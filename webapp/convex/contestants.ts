import { query, mutation } from "./_generated/server";
import { v } from "convex/values";

export const listContestants = query({
  args: {},
  handler: async (ctx) => {
    return await ctx.db.query("contestants").collect();
  },
});

export const listProperties = query({
  args: {},
  handler: async (ctx) => {
    return await ctx.db.query("properties").withIndex("by_order").collect();
  },
});

export const addContestant = mutation({
  args: { name: v.string() },
  handler: async (ctx, args) => {
    return await ctx.db.insert("contestants", {
      name: args.name,
      values: {},
    });
  },
});

export const addProperty = mutation({
  args: { 
    name: v.string(), 
    weight: v.number(), 
    higherIsBetter: v.boolean() 
  },
  handler: async (ctx, args) => {
    const properties = await ctx.db.query("properties").collect();
    const maxOrder = Math.max(...properties.map(p => p.order), -1);
    
    return await ctx.db.insert("properties", {
      name: args.name,
      weight: args.weight,
      higherIsBetter: args.higherIsBetter,
      order: maxOrder + 1,
    });
  },
});

export const updateContestantName = mutation({
  args: { 
    contestantId: v.id("contestants"), 
    name: v.string() 
  },
  handler: async (ctx, args) => {
    await ctx.db.patch(args.contestantId, { name: args.name });
  },
});

export const updateProperty = mutation({
  args: { 
    propertyId: v.id("properties"), 
    name: v.string(),
    weight: v.number(),
    higherIsBetter: v.boolean()
  },
  handler: async (ctx, args) => {
    await ctx.db.patch(args.propertyId, { 
      name: args.name,
      weight: args.weight,
      higherIsBetter: args.higherIsBetter
    });
  },
});

export const updateContestantValue = mutation({
  args: { 
    contestantId: v.id("contestants"), 
    propertyId: v.string(), 
    value: v.number() 
  },
  handler: async (ctx, args) => {
    const contestant = await ctx.db.get(args.contestantId);
    if (!contestant) throw new Error("Contestant not found");
    
    const newValues = { ...contestant.values };
    newValues[args.propertyId] = args.value;
    
    await ctx.db.patch(args.contestantId, { values: newValues });
  },
});

export const deleteContestant = mutation({
  args: { contestantId: v.id("contestants") },
  handler: async (ctx, args) => {
    await ctx.db.delete(args.contestantId);
  },
});

export const deleteProperty = mutation({
  args: { propertyId: v.id("properties") },
  handler: async (ctx, args) => {
    await ctx.db.delete(args.propertyId);
    
    // Clean up contestant values
    const contestants = await ctx.db.query("contestants").collect();
    for (const contestant of contestants) {
      const newValues = { ...contestant.values };
      delete newValues[args.propertyId];
      await ctx.db.patch(contestant._id, { values: newValues });
    }
  },
});

export const importData = mutation({
  args: { 
    contestants: v.array(v.object({
      name: v.string(),
      values: v.record(v.string(), v.number())
    })),
    properties: v.array(v.object({
      name: v.string(),
      weight: v.number(),
      higherIsBetter: v.boolean()
    }))
  },
  handler: async (ctx, args) => {
    // Clear existing data
    const existingContestants = await ctx.db.query("contestants").collect();
    const existingProperties = await ctx.db.query("properties").collect();
    
    for (const contestant of existingContestants) {
      await ctx.db.delete(contestant._id);
    }
    for (const property of existingProperties) {
      await ctx.db.delete(property._id);
    }
    
    // Insert new properties
    const propertyIdMap: Record<string, string> = {};
    for (let i = 0; i < args.properties.length; i++) {
      const property = args.properties[i];
      const id = await ctx.db.insert("properties", {
        name: property.name,
        weight: property.weight,
        higherIsBetter: property.higherIsBetter,
        order: i,
      });
      propertyIdMap[property.name] = id;
    }
    
    // Insert new contestants
    for (const contestant of args.contestants) {
      const mappedValues: Record<string, number> = {};
      for (const [propName, value] of Object.entries(contestant.values)) {
        if (propertyIdMap[propName]) {
          mappedValues[propertyIdMap[propName]] = value;
        }
      }
      
      await ctx.db.insert("contestants", {
        name: contestant.name,
        values: mappedValues,
      });
    }
  },
});
