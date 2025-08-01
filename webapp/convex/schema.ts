import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";
import { authTables } from "@convex-dev/auth/server";

const applicationTables = {
  contestants: defineTable({
    name: v.string(),
    values: v.record(v.string(), v.number()), // propertyId -> value
  }),
  
  properties: defineTable({
    name: v.string(),
    weight: v.number(),
    higherIsBetter: v.boolean(),
    order: v.number(),
  }).index("by_order", ["order"]),
};

export default defineSchema({
  ...authTables,
  ...applicationTables,
});
