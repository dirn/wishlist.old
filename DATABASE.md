# Database Schema Specification

## Overview

This document defines the database schema for the wishlist application. Each user has a single wishlist, represented by items that belong to them.

## Conventions

- **Timestamps**: `created_at` fields are set automatically when a record is created. `updated_at` fields are set automatically when a record is created and updated automatically each time the record is modified.

## Tables

### users

Stores user account information.

**Fields:**

- `id` - Unique identifier (UUID)
- `email` - User's email address (unique, required, case-insensitive)
- `password` - Hashed password (required)
- `created_at` - Account creation timestamp
- `updated_at` - Last update timestamp

**Relationships:**

- One-to-many with `items` (a user has many items in their wishlist)
- One-to-many with `wishlist_shares` as owner
- One-to-many with `wishlist_shares` as shared_with
- One-to-many with `claims`

### items

Stores wishlist items. Each user's wishlist is represented by the items that belong to them.

**Fields:**

- `id` - Unique identifier (UUID)
- `user_id` - Owner of the item (foreign key to `users.id`)
- `name` - Item name/title (required)
- `description` - Additional details about the item
- `link` - URL to the item (e.g., product page)
- `image` - Image URL (initially absolute paths like `/images/item-123.jpg`, can be extended with scheme/host for cloud storage at runtime)
- `priority` - Priority level (nullable integer, used for sorting; items with null priority appear after items with priority, and null priority items are sorted by created_at ascending)
- `quantity` - Desired quantity (integer, NULL means unlimited)
- `disabled` - Whether the item is disabled (boolean, default: false)
- `created_at` - Item creation timestamp
- `updated_at` - Last update timestamp
- `deleted_at` - Timestamp when item was deleted (nullable, NULL means not deleted)

**Relationships:**

- Many-to-one with `users` (each item belongs to one user)
- One-to-many with `claims` (an item can be claimed by multiple users)

### wishlist_shares

Tracks which users have shared their wishlist with other users.

**Fields:**

- `id` - Unique identifier (UUID)
- `owner_id` - User who owns the wishlist (foreign key to `users.id`)
- `shared_with_id` - User with whom the wishlist is shared (foreign key to `users.id`)
- `created_at` - Share creation timestamp
- `updated_at` - Last update timestamp

**Constraints:**

- Unique combination of `owner_id` and `shared_with_id`
- `owner_id` cannot equal `shared_with_id`

**Relationships:**

- Many-to-one with `users` as owner
- Many-to-one with `users` as shared_with

### claims

Tracks which users have claimed items from shared wishlists and whether they have been purchased.

**Fields:**

- `id` - Unique identifier (UUID)
- `item_id` - Item being claimed (foreign key to `items.id`)
- `user_id` - User making the claim (foreign key to `users.id`)
- `quantity` - Quantity claimed (integer, required, must be > 0)
- `quantity_purchased` - Quantity purchased (integer, default: 0, must be >= 0 and <= quantity)
- `created_at` - Claim creation timestamp
- `updated_at` - Last update timestamp

**Constraints:**

- Unique combination of `item_id` and `user_id` (a user can only have one claim per item)
- `quantity_purchased` cannot exceed `quantity`

**Relationships:**

- Many-to-one with `items` (an item can have multiple claims)
- Many-to-one with `users` (a user can have multiple claims)

## Indexes

**Planned indexes:**

- `items.user_id` - For querying a user's wishlist
- `items.disabled` - For filtering disabled items
- `wishlist_shares.owner_id` - For finding wishlists shared by a user
- `wishlist_shares.shared_with_id` - For finding wishlists shared with a user
- `claims.item_id` - For querying claims on an item
- `claims.user_id` - For querying a user's claims (shopping list)
