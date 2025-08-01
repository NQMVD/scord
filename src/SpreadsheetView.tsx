import { useQuery, useMutation } from "convex/react";
import { api } from "../convex/_generated/api";
import { useState, useMemo } from "react";
import { toast } from "sonner";
import { Id } from "../convex/_generated/dataModel";

interface ScoreResult {
  contestantId: Id<"contestants">;
  name: string;
  score: number;
  bestProperties: string[];
}

type ExportFormat = "json" | "csv";

export function SpreadsheetView() {
  const contestants = useQuery(api.contestants.listContestants) || [];
  const properties = useQuery(api.contestants.listProperties) || [];

  const addContestant = useMutation(api.contestants.addContestant);
  const addProperty = useMutation(api.contestants.addProperty);
  const updateContestantName = useMutation(
    api.contestants.updateContestantName,
  );
  const updateProperty = useMutation(api.contestants.updateProperty);
  const updateValue = useMutation(api.contestants.updateContestantValue);
  const deleteContestant = useMutation(api.contestants.deleteContestant);
  const deleteProperty = useMutation(api.contestants.deleteProperty);
  const importData = useMutation(api.contestants.importData);

  const [newContestantName, setNewContestantName] = useState("");
  const [newPropertyName, setNewPropertyName] = useState("");
  const [newPropertyWeight, setNewPropertyWeight] = useState(1);
  const [newPropertyHigherIsBetter, setNewPropertyHigherIsBetter] =
    useState(true);
  const [editingCell, setEditingCell] = useState<string | null>(null);
  const [editValue, setEditValue] = useState("");
  const [exportFormat, setExportFormat] = useState<ExportFormat>("json");

  // Calculate scores and find best properties
  const scoreResults: ScoreResult[] = useMemo(() => {
    if (contestants.length === 0 || properties.length === 0) return [];

    // Find who has the best value for each property
    const bestInProperty: Record<string, Set<string>> = {};

    for (const property of properties) {
      const allValues = contestants.map((c) => ({
        id: c._id,
        value: c.values[property._id] || 0,
      }));

      if (allValues.length === 0) continue;

      const bestValue = property.higherIsBetter
        ? Math.max(...allValues.map((v) => v.value))
        : Math.min(...allValues.map((v) => v.value));

      bestInProperty[property._id] = new Set(
        allValues.filter((v) => v.value === bestValue).map((v) => v.id),
      );
    }

    // Calculate normalized scores for each contestant
    const results: ScoreResult[] = [];

    for (const contestant of contestants) {
      let totalScore = 0;
      let totalWeight = 0;
      const bestProperties: string[] = [];

      for (const property of properties) {
        const value = contestant.values[property._id] || 0;

        // Check if this contestant has the best value for this property
        // Only include if they're the only one with the best value (no ties)
        if (
          bestInProperty[property._id]?.has(contestant._id) &&
          bestInProperty[property._id]?.size === 1
        ) {
          bestProperties.push(property.name);
        }

        // Get all values for this property to find min/max
        const allValues = contestants
          .map((c) => c.values[property._id] || 0)
          .filter((v) => !isNaN(v));

        if (allValues.length === 0) continue;

        const minValue = Math.min(...allValues);
        const maxValue = Math.max(...allValues);
        const range = maxValue - minValue;

        let normalizedScore: number;
        if (range === 0) {
          normalizedScore = 100;
        } else {
          if (property.higherIsBetter) {
            normalizedScore = ((value - minValue) / range) * 100;
          } else {
            normalizedScore = ((maxValue - value) / range) * 100;
          }
        }

        const weightedScore = normalizedScore * property.weight;
        totalScore += weightedScore;
        totalWeight += property.weight;
      }

      const finalScore = totalWeight > 0 ? totalScore / totalWeight : 0;

      results.push({
        contestantId: contestant._id,
        name: contestant.name,
        score: finalScore,
        bestProperties,
      });
    }

    return results.sort((a, b) => b.score - a.score);
  }, [contestants, properties]);

  const handleAddContestant = async () => {
    if (!newContestantName.trim()) return;
    try {
      await addContestant({ name: newContestantName.trim() });
      setNewContestantName("");
      toast.success("Contestant added");
    } catch (error) {
      toast.error("Failed to add contestant");
    }
  };

  const handleAddProperty = async () => {
    if (!newPropertyName.trim()) return;
    try {
      await addProperty({
        name: newPropertyName.trim(),
        weight: newPropertyWeight,
        higherIsBetter: newPropertyHigherIsBetter,
      });
      setNewPropertyName("");
      setNewPropertyWeight(1);
      setNewPropertyHigherIsBetter(true);
      toast.success("Property added");
    } catch (error) {
      toast.error("Failed to add property");
    }
  };

  const handleCellEdit = (
    type: "contestant" | "property" | "value",
    id: string,
    propertyId?: string,
    currentValue?: any,
  ) => {
    const cellKey = propertyId ? `${id}-${propertyId}` : `${type}-${id}`;
    setEditingCell(cellKey);
    setEditValue(currentValue?.toString() || "");
  };

  const handleCellSave = async (
    type: "contestant" | "property" | "value",
    id: string,
    propertyId?: string,
    field?: string,
  ) => {
    try {
      if (type === "contestant" && !propertyId) {
        if (!editValue.trim()) {
          toast.error("Name cannot be empty");
          return;
        }
        await updateContestantName({
          contestantId: id as Id<"contestants">,
          name: editValue.trim(),
        });
      } else if (type === "property" && field) {
        const actualId = id.replace("-weight", "");
        const property = properties.find((p) => p._id === actualId);
        if (!property) return;

        if (field === "name") {
          if (!editValue.trim()) {
            toast.error("Name cannot be empty");
            return;
          }
          await updateProperty({
            propertyId: actualId as Id<"properties">,
            name: editValue.trim(),
            weight: property.weight,
            higherIsBetter: property.higherIsBetter,
          });
        } else if (field === "weight") {
          const weight = parseFloat(editValue);
          if (isNaN(weight) || weight <= 0) {
            toast.error("Weight must be a positive number");
            return;
          }
          await updateProperty({
            propertyId: actualId as Id<"properties">,
            name: property.name,
            weight,
            higherIsBetter: property.higherIsBetter,
          });
        }
      } else if (type === "value" && propertyId) {
        const value = parseFloat(editValue);
        if (isNaN(value)) {
          toast.error("Invalid number");
          return;
        }
        await updateValue({
          contestantId: id as Id<"contestants">,
          propertyId,
          value,
        });
      }

      setEditingCell(null);
      toast.success("Updated successfully");
    } catch (error) {
      toast.error("Failed to update");
    }
  };

  const togglePropertyDirection = async (propertyId: Id<"properties">) => {
    const property = properties.find((p) => p._id === propertyId);
    if (!property) return;

    try {
      await updateProperty({
        propertyId,
        name: property.name,
        weight: property.weight,
        higherIsBetter: !property.higherIsBetter,
      });
      toast.success("Direction updated");
    } catch (error) {
      toast.error("Failed to update direction");
    }
  };

  const exportToCSV = (data: any[], filename: string) => {
    if (data.length === 0) return;

    const headers = Object.keys(data[0]);
    const csvContent = [
      headers.join(","),
      ...data.map((row) =>
        headers
          .map((header) => {
            const value = row[header];
            if (Array.isArray(value)) {
              return `"${value.join("; ")}"`;
            }
            return typeof value === "string" && value.includes(",")
              ? `"${value}"`
              : value;
          })
          .join(","),
      ),
    ].join("\n");

    const blob = new Blob([csvContent], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  };

  const exportToJSON = (data: any, filename: string) => {
    const blob = new Blob([JSON.stringify(data, null, 2)], {
      type: "application/json",
    });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleExportData = () => {
    const exportData = {
      contestants: contestants.map((c) => ({
        name: c.name,
        values: Object.fromEntries(
          Object.entries(c.values).map(([propId, value]) => {
            const prop = properties.find((p) => p._id === propId);
            return [prop?.name || propId, value];
          }),
        ),
      })),
      properties: properties.map((p) => ({
        name: p.name,
        weight: p.weight,
        higherIsBetter: p.higherIsBetter,
      })),
    };

    if (exportFormat === "csv") {
      exportToCSV(exportData.contestants, "contestant-data.csv");
      exportToCSV(exportData.properties, "properties-data.csv");
      toast.success("Data exported as CSV files");
    } else {
      exportToJSON(exportData, "contestant-data.json");
      toast.success("Data exported as JSON");
    }
  };

  const handleExportResults = () => {
    const resultsData = scoreResults.map((result) => ({
      rank: scoreResults.indexOf(result) + 1,
      name: result.name,
      score: parseFloat(result.score.toFixed(1)),
      bestProperties: result.bestProperties,
    }));

    if (exportFormat === "csv") {
      exportToCSV(resultsData, "scoring-results.csv");
    } else {
      exportToJSON(resultsData, "scoring-results.json");
    }
    toast.success("Results exported");
  };

  const handleImport = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = async (e) => {
      try {
        const data = JSON.parse(e.target?.result as string);
        await importData(data);
        toast.success("Data imported");
      } catch (error) {
        toast.error("Failed to import data");
      }
    };
    reader.readAsText(file);
    event.target.value = "";
  };

  return (
    <div className="flex gap-6 h-full">
      {/* Main spreadsheet area */}
      <div className="flex-1 charcoal-surface rounded-lg p-6">
        {/* <div className="flex-1 charcoal-surface rounded-lg p-6"> */}
        <div className="mb-6">
          <h2 className="text-2xl font-bold charcoal-glow-text mb-4">
            Contestant Data
          </h2>

          {/* Controls */}
          <div className="flex flex-wrap gap-4 mb-4">
            <div className="flex gap-2">
              <input
                type="text"
                placeholder="Contestant name"
                value={newContestantName}
                onChange={(e) => setNewContestantName(e.target.value)}
                className="px-3 py-2 bg-charcoal-950 border border-charcoal-800 rounded text-charcoal-100 placeholder-charcoal-400 transition-all duration-300 hover:shadow-glow focus:shadow-glow-hover"
                onKeyDown={(e) => e.key === "Enter" && handleAddContestant()}
              />
              <button
                onClick={handleAddContestant}
                className="px-4 py-2 charcoal-surface text-charcoal-100 rounded interactive-element"
              >
                Add Contestant
              </button>
            </div>

            <div className="flex gap-2">
              <input
                type="text"
                placeholder="Property name"
                value={newPropertyName}
                onChange={(e) => setNewPropertyName(e.target.value)}
                className="px-3 py-2 bg-charcoal-950 border border-charcoal-800 rounded text-charcoal-100 placeholder-charcoal-400 transition-all duration-300 hover:shadow-glow focus:shadow-glow-hover"
              />
              <input
                type="number"
                placeholder="Weight"
                value={newPropertyWeight}
                onChange={(e) => setNewPropertyWeight(Number(e.target.value))}
                className="w-20 px-3 py-2 bg-charcoal-950 border border-charcoal-800 rounded text-charcoal-100 transition-all duration-300 hover:shadow-glow focus:shadow-glow-hover"
              />
              <label className="flex items-center gap-2 text-charcoal-300">
                <input
                  type="checkbox"
                  checked={newPropertyHigherIsBetter}
                  onChange={(e) =>
                    setNewPropertyHigherIsBetter(e.target.checked)
                  }
                  className="rounded"
                />
                Higher is better
              </label>
              <button
                onClick={handleAddProperty}
                className="px-4 py-2 charcoal-surface text-charcoal-100 rounded interactive-element"
              >
                Add Property
              </button>
            </div>

            <div className="flex gap-2 items-center">
              <select
                value={exportFormat}
                onChange={(e) =>
                  setExportFormat(e.target.value as ExportFormat)
                }
                className="px-3 py-2 bg-charcoal-950 border border-charcoal-800 rounded text-charcoal-100 transition-all duration-300 hover:shadow-glow"
              >
                <option value="json">JSON</option>
                <option value="csv">CSV</option>
              </select>
              <button
                onClick={handleExportData}
                className="px-4 py-2 charcoal-surface text-charcoal-100 rounded interactive-element"
              >
                Export Data
              </button>
              <button
                onClick={handleExportResults}
                className="px-4 py-2 charcoal-surface text-charcoal-100 rounded interactive-element"
              >
                Export Results
              </button>
              <label className="px-4 py-2 charcoal-surface text-charcoal-100 rounded interactive-element cursor-pointer">
                Import
                <input
                  type="file"
                  accept=".json"
                  onChange={handleImport}
                  className="hidden"
                />
              </label>
            </div>
          </div>
        </div>

        {/* Spreadsheet table */}
        <div className="overflow-auto">
          <table className="w-full border-collapse">
            <thead>
              <tr className="bg-charcoal-900 shadow-inner">
                <th className="border border-charcoal-800 px-4 py-2 text-left text-charcoal-100 shadow-inner">
                  Contestant
                </th>
                {properties.map((property) => {
                  const isEditingName =
                    editingCell === `property-${property._id}`;
                  const isEditingWeight =
                    editingCell === `property-${property._id}-weight`;

                  return (
                    <th
                      key={property._id}
                      className="border border-charcoal-800 px-4 py-2 text-center text-charcoal-100 min-w-32 shadow-inner"
                    >
                      <div className="flex flex-col gap-1">
                        {isEditingName ? (
                          <input
                            type="text"
                            value={editValue}
                            onChange={(e) => setEditValue(e.target.value)}
                            onBlur={() =>
                              handleCellSave(
                                "property",
                                property._id,
                                undefined,
                                "name",
                              )
                            }
                            onKeyDown={(e) => {
                              if (e.key === "Enter")
                                handleCellSave(
                                  "property",
                                  property._id,
                                  undefined,
                                  "name",
                                );
                              if (e.key === "Escape") setEditingCell(null);
                            }}
                            className="px-2 py-1 bg-charcoal-800 border border-charcoal-600 rounded text-charcoal-100 text-center shadow-glow"
                            autoFocus
                          />
                        ) : (
                          <button
                            onClick={() =>
                              handleCellEdit(
                                "property",
                                property._id,
                                undefined,
                                property.name,
                              )
                            }
                            className="hover:bg-charcoal-800 rounded px-2 py-1 transition-all duration-300 hover:shadow-glow"
                          >
                            {property.name}
                          </button>
                        )}

                        <div className="text-xs text-charcoal-400 flex items-center justify-center gap-2">
                          <span>Weight:</span>
                          {isEditingWeight ? (
                            <input
                              type="number"
                              value={editValue}
                              onChange={(e) => setEditValue(e.target.value)}
                              onBlur={() =>
                                handleCellSave(
                                  "property",
                                  property._id,
                                  undefined,
                                  "weight",
                                )
                              }
                              onKeyDown={(e) => {
                                if (e.key === "Enter")
                                  handleCellSave(
                                    "property",
                                    property._id,
                                    undefined,
                                    "weight",
                                  );
                                if (e.key === "Escape") setEditingCell(null);
                              }}
                              className="w-12 px-1 py-0 bg-charcoal-800 border border-charcoal-600 rounded text-charcoal-100 text-center text-xs shadow-glow"
                              autoFocus
                            />
                          ) : (
                            <button
                              onClick={() =>
                                handleCellEdit(
                                  "property",
                                  property._id + "-weight",
                                  undefined,
                                  property.weight,
                                )
                              }
                              className="hover:bg-charcoal-800 rounded px-1 transition-all duration-300 hover:shadow-glow"
                            >
                              {property.weight}
                            </button>
                          )}

                          <button
                            onClick={() =>
                              togglePropertyDirection(property._id)
                            }
                            className="hover:bg-charcoal-800 rounded px-1 transition-all duration-300 hover:shadow-glow"
                            title={`Click to change to ${property.higherIsBetter ? "lower" : "higher"} is better`}
                          >
                            {property.higherIsBetter ? "↑" : "↓"}
                          </button>
                        </div>

                        <button
                          onClick={() =>
                            deleteProperty({ propertyId: property._id })
                          }
                          className="text-xs text-charcoal-500 hover:text-charcoal-400 transition-colors"
                        >
                          Delete
                        </button>
                      </div>
                    </th>
                  );
                })}
                <th className="border border-charcoal-800 px-4 py-2 text-center text-charcoal-100 shadow-inner">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody>
              {contestants.map((contestant) => {
                const isEditingName =
                  editingCell === `contestant-${contestant._id}`;

                return (
                  <tr
                    key={contestant._id}
                    className="hover:bg-charcoal-900 transition-all duration-300 hover:shadow-inner"
                  >
                    <td className="border border-charcoal-800 px-4 py-2 font-medium text-charcoal-100">
                      {isEditingName ? (
                        <input
                          type="text"
                          value={editValue}
                          onChange={(e) => setEditValue(e.target.value)}
                          onBlur={() =>
                            handleCellSave("contestant", contestant._id)
                          }
                          onKeyDown={(e) => {
                            if (e.key === "Enter")
                              handleCellSave("contestant", contestant._id);
                            if (e.key === "Escape") setEditingCell(null);
                          }}
                          className="w-full px-2 py-1 bg-charcoal-800 border border-charcoal-800 rounded text-charcoal-100 shadow-glow"
                          autoFocus
                        />
                      ) : (
                        <button
                          onClick={() =>
                            handleCellEdit(
                              "contestant",
                              contestant._id,
                              undefined,
                              contestant.name,
                            )
                          }
                          className="w-full text-left hover:bg-charcoal-800 rounded px-2 py-1 transition-all duration-300 hover:shadow-glow"
                        >
                          {contestant.name}
                        </button>
                      )}
                    </td>
                    {properties.map((property) => {
                      const cellKey = `${contestant._id}-${property._id}`;
                      const value = contestant.values[property._id] || 0;
                      const isEditing = editingCell === cellKey;

                      return (
                        <td
                          key={property._id}
                          className="border border-charcoal-800 px-2 py-2 text-center"
                        >
                          {isEditing ? (
                            <input
                              type="number"
                              value={editValue}
                              onChange={(e) => setEditValue(e.target.value)}
                              onBlur={() =>
                                handleCellSave(
                                  "value",
                                  contestant._id,
                                  property._id,
                                )
                              }
                              onKeyDown={(e) => {
                                if (e.key === "Enter")
                                  handleCellSave(
                                    "value",
                                    contestant._id,
                                    property._id,
                                  );
                                if (e.key === "Escape") setEditingCell(null);
                              }}
                              className="w-full px-2 py-1 bg-charcoal-800 border border-charcoal-800 rounded text-charcoal-100 text-center shadow-glow"
                              autoFocus
                            />
                          ) : (
                            <button
                              onClick={() =>
                                handleCellEdit(
                                  "value",
                                  contestant._id,
                                  property._id,
                                  value,
                                )
                              }
                              className="w-full px-2 py-1 hover:bg-charcoal-800 rounded text-charcoal-100 transition-all duration-300 hover:shadow-glow"
                            >
                              {value}
                            </button>
                          )}
                        </td>
                      );
                    })}
                    <td className="border border-charcoal-800 px-4 py-2 text-center">
                      <button
                        onClick={() =>
                          deleteContestant({ contestantId: contestant._id })
                        }
                        className="text-charcoal-500 hover:text-charcoal-400 text-sm transition-colors"
                      >
                        Delete
                      </button>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>

      {/* Results panel */}
      <div className="w-80 charcoal-surface rounded-lg p-6">
        <h3 className="text-xl font-bold charcoal-glow-text mb-4">
          Scoring Results
        </h3>

        <div className="space-y-4">
          {scoreResults.map((result, index) => (
            <div
              key={result.contestantId}
              className="bg-charcoal-900 rounded-lg p-4 shadow-charcoal hover:shadow-charcoal-hover transition-all duration-300 interactive-element"
            >
              <div className="flex justify-between items-center mb-2">
                <span className="font-medium text-charcoal-100">
                  #{index + 1} {result.name}
                </span>
                <span className="text-lg font-bold text-charcoal-100 charcoal-glow-text">
                  {result.score.toFixed(1)}%
                </span>
              </div>

              {result.bestProperties.length > 0 && (
                <div className="text-sm text-charcoal-300">
                  <span className="text-charcoal-400">Best at:</span>{" "}
                  {result.bestProperties.join(", ")}
                </div>
              )}
            </div>
          ))}
        </div>

        {scoreResults.length === 0 && (
          <div className="text-center text-charcoal-500 mt-8">
            Add contestants and properties to see scoring results
          </div>
        )}
      </div>
    </div>
  );
}
