# Wishlist Web App - Specification

## Overview
A web application that allows users to create and manage their personal wishlists. Users can share their wishlists with other authenticated users.

---

## Core Functionality

### User Management
- **Multiple Users**: The application supports multiple users, each with their own account
- **User Authentication**: Users must be able to register and log in
- **User Isolation**: Each user has their own personal wishlist

### Wishlist Management
- **Single Wishlist per User**: Each user has one wishlist (not multiple lists)
- **Wishlist Sharing**: Users can share their wishlist with other users
- **Shared Wishlist Access**: 
  - Only authenticated users can view shared wishlists
  - Users must be logged in to access a shared wishlist link

### Wishlist Items
Each item in a wishlist must support the following fields:
- **Name** (required): The name/title of the item
- **Description**: Additional details about the item
- **Link**: URL to the item (e.g., product page, store link)
- **Image**: Image/photo of the item
- **Priority**: Priority level of the item
- **Quantity**: Desired quantity of the item
  - Default quantity: 1
  - Users can specify a specific quantity (e.g., 2, 5, 10)
  - Users can mark items as "unlimited" (no quantity limit)
  - Unlimited must be explicitly set by the user (not the default)

### Wishlist Operations
- **View Own Wishlist**: Users can view all items in their own wishlist
- **Edit Own Wishlist**: Users can add and remove items from their own wishlist
- **Disable Items**: Users can disable items in their wishlist (items remain but are marked as disabled)

### Shared Wishlist Viewing
- **View Shared Wishlists**: Users can see a list of all wishlists that have been shared with them
- **View Shared Wishlist Details**: Users can view the contents of a shared wishlist
- **Claim Items**: When viewing a shared wishlist, users can claim items they intend to purchase
  - Users can claim a quantity less than or equal to the item's available quantity
  - Multiple users can claim the same item
  - For items with a specific quantity: total claims cannot exceed the desired quantity
  - For unlimited items: any number of users can claim any quantity
  - Items can be partially claimed (e.g., if 5 are desired and 2 are claimed, 3 remain available)
- **Hide Claimed Items**: 
  - Enhancement: Items that are fully claimed (no remaining available quantity) can be hidden from view when browsing shared wishlists
  - Toggle option to show/hide fully claimed items
  - Note: Items that are partially claimed remain visible (they still have available quantity)
- **Privacy**: Wishlist owners cannot see which items have been claimed or purchased by others

### Shopping List
- **Shopping List View**: Items that have been claimed by the user appear on their personal shopping list
- **Mark as Purchased**: Users can mark claimed items as purchased
  - Purchased items move from shopping list to a "purchased items" list
  - Purchased items are tracked separately from claimed items

### Item Status Management
- **Item Availability**: Items have an availability status based on claims and purchases:
  - Items can be both **available** and **claimed** simultaneously
  - Example: If an item has a desired quantity of 5, and 2 are claimed, the item is both:
    - Partially claimed (2 of 5)
    - Still available (3 remaining)
  - An item is only "fully claimed" when the total claimed + purchased quantity equals or exceeds the desired quantity (or if unlimited, never fully claimed)
- **Item States**: Items can have the following additional states:
  - **Disabled** (by wishlist owner): Item is disabled but remains in the wishlist
  - **Deleted** (by wishlist owner): Item is removed from the wishlist
- **Claim/Purchase Tracking**: 
  - Each item tracks how many have been claimed (by whom and in what quantity)
  - Each item tracks how many have been purchased (by whom and in what quantity)
  - Available quantity = Desired quantity - (Claimed quantity + Purchased quantity)
  - For unlimited items: available quantity is always unlimited

### Notifications & Edge Cases
- **Item Deletion/Disable Notification**: 
  - If a wishlist owner deletes or disables an item that has been claimed, the user(s) who claimed it must be notified
  - Notification method: TBD (in-app notification, email, or both)
- **Quantity Reduction Handling**:
  - If a wishlist owner reduces the desired quantity of an item (or changes from unlimited to a specific quantity):
    - If total claims + purchases exceed the new desired quantity:
      - Prioritize items that have been purchased (these remain valid)
      - Notify users who have only claimed (not purchased) items first
      - Handle edge case where sum of all claims exceeds requested amount
      - System should handle over-claiming scenarios gracefully
  - If a wishlist owner changes an item from a specific quantity to unlimited:
    - All existing claims and purchases remain valid
    - Item becomes fully available for additional claims

---

## Status
- ✅ Core Functionality - Defined
- ✅ User Experience - Defined
- ⏳ Additional Features - Pending
- ⏳ Technical Requirements - Pending
