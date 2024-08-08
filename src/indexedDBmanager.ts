let db: IDBDatabase | null;
let inactivityTimer: number;

// Open the database
export function openDatabase() {
  const request = indexedDB.open("mydatabase", 1);

  request.onsuccess = function (event: Event) {
    db = (event.target as IDBOpenDBRequest).result as IDBDatabase;
    console.log("Database opened successfully.");
    startInactivityTimer();
  };

  request.onerror = function (event: Event) {
    console.error("Database error:", (event.target as IDBOpenDBRequest).error);
  };

  request.onupgradeneeded = function (event: IDBVersionChangeEvent) {
    db = (event.target as IDBOpenDBRequest).result as IDBDatabase;
    if (!db.objectStoreNames.contains("myStore")) {
      db.createObjectStore("myStore", { keyPath: "id" });
    }
  };
}

// Close the database
export function closeDatabase() {
  if (db) {
    db.close();
    console.log("Database closed due to inactivity.");
    db = null; // Clear the reference to prevent accidental usage
  }
}

// Start the inactivity timer
function startInactivityTimer() {
  clearTimeout(inactivityTimer);
  inactivityTimer = setTimeout(closeDatabase, 5 * 60 * 1000); // 5 minutes
}

// Reset the timer on user activity
export function resetInactivityTimer() {
  if (db) {
    startInactivityTimer();
  }
}
