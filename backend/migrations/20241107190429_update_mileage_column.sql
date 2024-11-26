-- Step 1: Update existing NULL values to a default value (e.g., 0)
UPDATE expenses
SET mileage = 0
WHERE mileage IS NULL;

-- Step 2: Alter the column to NOT NULL
ALTER TABLE expenses
ALTER COLUMN mileage SET NOT NULL;