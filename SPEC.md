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

---

## Status
- ✅ Core Functionality - Defined
- ⏳ User Experience - Pending
- ⏳ Additional Features - Pending
- ⏳ Technical Requirements - Pending
