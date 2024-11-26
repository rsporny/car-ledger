# Car Ledger

## Overview

Car Ledger is a vehicle expense and history tracking app designed for individual car owners, EV/hybrid users, and fleet managers.
Unlike traditional car journal apps, our solution leverages blockchain to securely record each vehicle’s financial history, creating a tamper-proof,
transparent record of all expenses. This blockchain-powered feature uniquely positions the app as a value-enhancing tool in the used car market,
where transparent records can increase resale value and trust between buyers and sellers.

This repository contains both the backend and frontend for the Car Ledger application.

## Problem Statement

In the automotive industry, transparency in vehicle history is a critical factor in resale value. Buyers and sellers of used vehicles often lack
a reliable way to verify historical expenses, maintenance records, and overall ownership costs, leading to concerns about fraud and inflated resale prices.
Traditional car history reports cover only a portion of a vehicle’s life, primarily accidents and basic maintenance, leaving a gap in fully
documenting ownership costs.

## Our Solution

Car Ledger offers a digital, blockchain-based car journal that records all vehicle-related expenses, from fuel and maintenance to insurance and EV
charging costs. This comprehensive expense tracking:
* Provides Full Transparency: Offers used car buyers a complete financial history of the vehicle, creating trust and potentially increasing resale value.
* Ensures Security and Privacy: Through blockchain technology, the app guarantees that records cannot be altered or deleted, preventing tampering and providing a trusted record for all parties.
* Supports All Vehicles: Including traditional fuel cars, EVs, and plug-in hybrids, making it a versatile solution in today’s diversified automotive market.

## Technology

### Backend

The backend is built using Actix Web and SQLx for PostgreSQL integration.

#### Running the Backend

1. Navigate to the `backend` directory:
    ```sh
    cd backend
    ```
2. Create a `.env` file with your database URL:
    ```env
    DATABASE_URL=postgres://username:password@localhost/car_ledger_db
    ```
3. Run the database migrations:
    ```sh
    sqlx migrate run
    ```
4. Run the backend:
    ```sh
    cargo run
    ```

### Frontend

The frontend is built using the Yew web framework.

#### Running the Frontend

1. Navigate to the `frontend` directory:
    ```sh
    cd frontend
    ```
2. Install `trunk` if you haven't already:
    ```sh
    cargo install trunk
    ```
3. Run the frontend:
    ```sh
    trunk serve
    ```
