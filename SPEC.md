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

### Wishlist Operations
- **View Own Wishlist**: Users can view all items in their own wishlist
- **Edit Own Wishlist**: Users can add and remove items from their own wishlist
- **Disable Items**: Users can disable items in their wishlist (items remain but are marked as disabled)

### Shared Wishlist Viewing
- **View Shared Wishlists**: Users can see a list of all wishlists that have been shared with them
- **View Shared Wishlist Details**: Users can view the contents of a shared wishlist
- **Claim Items**: When viewing a shared wishlist, users can claim items they intend to purchase
  - Users can claim a quantity less than or equal to the item's desired quantity
  - Multiple users can claim the same item (up to the desired quantity)
- **Hide Claimed Items**: 
  - Enhancement: Claimed items can be hidden from view when browsing shared wishlists
  - Toggle option to show/hide claimed items
- **Privacy**: Wishlist owners cannot see which items have been claimed or purchased by others

### Shopping List
- **Shopping List View**: Items that have been claimed by the user appear on their personal shopping list
- **Mark as Purchased**: Users can mark claimed items as purchased
  - Purchased items move from shopping list to a "purchased items" list
  - Purchased items are tracked separately from claimed items

### Item Status Management
- **Item States**: Items can be in the following states:
  - Available (not claimed)
  - Claimed (by one or more users)
  - Purchased (by one or more users)
  - Disabled (by wishlist owner)
  - Deleted (by wishlist owner)

### Notifications & Edge Cases
- **Item Deletion/Disable Notification**: 
  - If a wishlist owner deletes or disables an item that has been claimed, the user(s) who claimed it must be notified
  - Notification method: TBD (in-app notification, email, or both)
- **Quantity Reduction Handling**:
  - If a wishlist owner reduces the desired quantity of an item:
    - If total claims + purchases exceed the new desired quantity:
      - Prioritize items that have been purchased (these remain valid)
      - Notify users who have only claimed (not purchased) items first
      - Handle edge case where sum of all claims exceeds requested amount
      - System should handle over-claiming scenarios gracefully

---

## Status
- ✅ Core Functionality - Defined
- ✅ User Experience - Defined
- ⏳ Additional Features - Pending
- ⏳ Technical Requirements - Pending
