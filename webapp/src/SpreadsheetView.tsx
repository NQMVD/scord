import { useState, useMemo, useEffect } from "react";
import { toast } from "sonner";
import Files from "./assets/glass icons/icons/files";
import AwardTranslucent from "./assets/glass icons/icons/award-translucent";
import CloudDownload from "./assets/glass icons/icons/cloud-download";

// Local types to replace Convex-generated ones
interface Contestant {
  _id: string;
  name: string;
  values: Record<string, number>;
}

interface Property {
  _id: string;
  name: string;
  weight: number;
  higherIsBetter: boolean;
}

interface ScoreResult {
  contestantId: string;
  name: string;
  score: number;
  bestProperties: string[];
}

type ExportFormat = "json" | "csv";

// Local storage keys
const CONTESTANTS_KEY = "scord-contestants";
const PROPERTIES_KEY = "scord-properties";

// Helper functions for localStorage
const loadFromStorage = <T,>(key: string, defaultValue: T): T => {
  try {
    const item = localStorage.getItem(key);
    return item ? JSON.parse(item) : defaultValue;
  } catch {
    return defaultValue;
  }
};

const saveToStorage = <T,>(key: string, value: T): void => {
  try {
    localStorage.setItem(key, JSON.stringify(value));
  } catch (error) {
    console.error(`Failed to save to localStorage:`, error);
  }
};

export function SpreadsheetView() {
  // Local state for data
  const [contestants, setContestants] = useState<Contestant[]>([]);
  const [properties, setProperties] = useState<Property[]>([]);

  // UI state
  const [newContestantName, setNewContestantName] = useState("");
  const [newPropertyName, setNewPropertyName] = useState("");
  const [newPropertyWeight, setNewPropertyWeight] = useState(1);
  const [newPropertyHigherIsBetter, setNewPropertyHigherIsBetter] =
    useState(true);
  const [editingCell, setEditingCell] = useState<string | null>(null);
  const [editValue, setEditValue] = useState("");
  const [exportFormat, setExportFormat] = useState<ExportFormat>("json");

  // Load data from localStorage on mount
  useEffect(() => {
    setContestants(loadFromStorage(CONTESTANTS_KEY, []));
    setProperties(loadFromStorage(PROPERTIES_KEY, []));
  }, []);

  // Save to localStorage whenever data changes
  useEffect(() => {
    saveToStorage(CONTESTANTS_KEY, contestants);
  }, [contestants]);

  useEffect(() => {
    saveToStorage(PROPERTIES_KEY, properties);
  }, [properties]);

  // Generate unique IDs
  const generateId = () => Math.random().toString(36).substring(2, 15);

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

  // Data manipulation functions
  const handleAddContestant = () => {
    if (!newContestantName.trim()) return;
    try {
      const newContestant: Contestant = {
        _id: generateId(),
        name: newContestantName.trim(),
        values: {},
      };
      setContestants((prev) => [...prev, newContestant]);
      setNewContestantName("");
      toast.success("Contestant added");
    } catch (error) {
      toast.error("Failed to add contestant");
    }
  };

  const handleAddProperty = () => {
    if (!newPropertyName.trim()) return;
    try {
      const newProperty: Property = {
        _id: generateId(),
        name: newPropertyName.trim(),
        weight: newPropertyWeight,
        higherIsBetter: newPropertyHigherIsBetter,
      };
      setProperties((prev) => [...prev, newProperty]);
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

  const handleCellSave = (
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
        setContestants((prev) =>
          prev.map((c) =>
            c._id === id ? { ...c, name: editValue.trim() } : c,
          ),
        );
      } else if (type === "property" && field) {
        const actualId = id.replace("-weight", "");
        const property = properties.find((p) => p._id === actualId);
        if (!property) return;

        if (field === "name") {
          if (!editValue.trim()) {
            toast.error("Name cannot be empty");
            return;
          }
          setProperties((prev) =>
            prev.map((p) =>
              p._id === actualId ? { ...p, name: editValue.trim() } : p,
            ),
          );
        } else if (field === "weight") {
          const weight = parseFloat(editValue);
          if (isNaN(weight) || weight <= 0) {
            toast.error("Weight must be a positive number");
            return;
          }
          setProperties((prev) =>
            prev.map((p) => (p._id === actualId ? { ...p, weight } : p)),
          );
        }
      } else if (type === "value" && propertyId) {
        const value = parseFloat(editValue);
        if (isNaN(value)) {
          toast.error("Invalid number");
          return;
        }
        setContestants((prev) =>
          prev.map((c) =>
            c._id === id
              ? { ...c, values: { ...c.values, [propertyId]: value } }
              : c,
          ),
        );
      }

      setEditingCell(null);
      toast.success("Updated successfully");
    } catch (error) {
      toast.error("Failed to update");
    }
  };

  const togglePropertyDirection = (propertyId: string) => {
    const property = properties.find((p) => p._id === propertyId);
    if (!property) return;

    try {
      setProperties((prev) =>
        prev.map((p) =>
          p._id === propertyId
            ? { ...p, higherIsBetter: !p.higherIsBetter }
            : p,
        ),
      );
      toast.success("Direction updated");
    } catch (error) {
      toast.error("Failed to update direction");
    }
  };

  const deleteContestant = (contestantId: string) => {
    setContestants((prev) => prev.filter((c) => c._id !== contestantId));
    toast.success("Contestant deleted");
  };

  const deleteProperty = (propertyId: string) => {
    setProperties((prev) => prev.filter((p) => p._id !== propertyId));
    // Also remove values for this property from all contestants
    setContestants((prev) =>
      prev.map((c) => {
        const { [propertyId]: removed, ...restValues } = c.values;
        return { ...c, values: restValues };
      }),
    );
    toast.success("Property deleted");
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
    reader.onload = (e) => {
      try {
        const data = JSON.parse(e.target?.result as string);
        
        // Create new contestants with generated IDs
        if (data.contestants && Array.isArray(data.contestants)) {
          const newContestants: Contestant[] = data.contestants.map((c: any) => ({
            _id: generateId(),
            name: c.name || "Unnamed",
            values: {},
          }));
          setContestants(newContestants);
        }

        // Create new properties with generated IDs
        if (data.properties && Array.isArray(data.properties)) {
          const newProperties: Property[] = data.properties.map((p: any) => ({
            _id: generateId(),
            name: p.name || "Unnamed Property",
            weight: p.weight || 1,
            higherIsBetter: p.higherIsBetter !== false,
          }));
          setProperties(newProperties);

          // Update contestants with values for the new properties
          setContestants((prevContestants) =>
            prevContestants.map((contestant, cIndex) => {
              const importedContestant = data.contestants[cIndex];
              if (!importedContestant?.values) return contestant;

              const newValues: Record<string, number> = {};
              newProperties.forEach((property) => {
                const originalProperty = data.properties.find((p: any) => p.name === property.name);
                if (originalProperty && importedContestant.values[originalProperty.name] !== undefined) {
                  newValues[property._id] = importedContestant.values[originalProperty.name];
                }
              });

              return { ...contestant, values: newValues };
            }),
          );
        }

        toast.success("Data imported");
      } catch (error) {
        toast.error("Failed to import data");
      }
    };
    reader.readAsText(file);
    event.target.value = "";
  };

  return (
    <div className="flex gap-6 h-full max-w-full">
      {/* Main spreadsheet area */}
      <div className="flex-1 charcoal-surface rounded-lg p-6 flex flex-col min-w-0">
        <div className="mb-6 flex-shrink-0">
          <h2 className="text-2xl font-bold charcoal-glow-text mb-4">
            Contestant Data
          </h2>

          {/* Controls */}
          <div className="flex justify-between items-start mb-4">
            {/* Left side - Input controls */}
            <div className="space-y-4">
              {/* Contestant controls */}
              <div className="flex gap-2 items-center">
                <span className="text-charcoal-300 w-12">New:</span>
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

              {/* Property controls */}
              <div className="flex gap-2 items-center flex-wrap">
                <span className="text-charcoal-300 w-12">New:</span>
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
            </div>

            {/* Right side - Import/Export controls */}
            <div className="flex flex-col gap-2 items-end">
              <div className="flex gap-2 items-center">
                <span className="text-charcoal-300">Format:</span>
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
              </div>
              <button
                onClick={handleExportData}
                className="px-4 py-2 charcoal-surface text-charcoal-100 rounded interactive-element bg-charcoal-800 flex items-center gap-2"
              >
                <Files className="w-5 h-5" />
                Export Data
              </button>
              <button
                onClick={handleExportResults}
                className="px-4 py-2 charcoal-surface text-charcoal-100 rounded interactive-element bg-charcoal-700 flex items-center gap-2"
              >
                <AwardTranslucent className="w-5 h-5" />
                Export Results
              </button>
              <label className="px-4 py-2 charcoal-surface text-charcoal-100 rounded interactive-element cursor-pointer flex items-center gap-2">
                <CloudDownload className="w-5 h-5" />
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
        <div className="flex-1 overflow-auto max-w-full">
          <table className="border-collapse" style={{ minWidth: "100%" }}>
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
                          onClick={() => deleteProperty(property._id)}
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
                        onClick={() => deleteContestant(contestant._id)}
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
      <div className="w-80 charcoal-surface rounded-lg p-6 flex flex-col">
        <h2 className="text-2xl font-bold charcoal-glow-text mb-4 flex-shrink-0">
          Scoring Results
        </h2>

        <div className="space-y-4 flex-1 overflow-auto">
          {scoreResults.map((result, index) => (
            <div
              key={result.contestantId}
              className="bg-charcoal-900 rounded-lg p-4 shadow-charcoal hover:shadow-charcoal-hover transition-all duration-300 interactive-element"
            >
              <div className="flex justify-between items-center mb-2">
                <span className="font-medium text-charcoal-100">
                  <span className="text-charcoal-400">#{index + 1}</span>
                  <span className="ml-2">{result.name}</span>
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