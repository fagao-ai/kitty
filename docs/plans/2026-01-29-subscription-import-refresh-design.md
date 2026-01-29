# Subscription Import and Refresh Feature Design

**Date:** 2026-01-29
**Status:** Implemented

## Overview

This document describes the design and implementation of subscription import and refresh functionality for Xray proxies in the kitty application.

## Background

The kitty application is a Tauri-based proxy manager that supports both Xray and Hysteria2 protocols. Users need the ability to:
- Import proxy configurations from subscription URLs
- Refresh existing subscriptions to get updated proxy lists

This feature is implemented for Xray proxies only, as Hysteria2 does not use subscriptions.

## Architecture

### Data Model

The implementation uses two main database entities:

**subscribe Table:**
- `id`: Primary key (auto-increment)
- `url`: Subscription URL (unique constraint enforced at application level)

**xray Table:**
- `id`: Primary key
- `name`: Proxy name
- `protocol`: Protocol type (vless/vmess/trojan)
- `uuid`: User identifier
- `address`: Server address
- `port`: Server port
- `stream_settings`: Transport settings (JSON)
- `subscribe_id`: Foreign key to subscribe table (optional)

### API Endpoints

Two Tauri commands are implemented in [proxy.rs](../../src-tauri/src/tauri_apis/proxy.rs):

1. **`import_subscription(url: String)`**
   - Imports a new subscription from HTTP/HTTPS URL
   - Creates subscription record and associated proxy records
   - Returns error if URL already exists

2. **`refresh_subscription(record_ids: Option<Vec<i32>>)`**
   - Refreshes existing subscriptions
   - Updates proxy records for specified subscriptions, or all if none specified
   - Continues processing other subscriptions if one fails

### Supporting Components

**Subscription Parser** ([parse_subscription.rs](../../src-tauri/src/apis/parse_subscription.rs)):
- `download_subcriptions(url)`: Downloads and decodes base64-encoded subscription content
- Returns list of proxy URLs with protocol type

**Entity Models** ([entity/src/](../../src-tauri/entity/src/)):
- `xray::Model::from_str()`: Parses proxy URL into database model
- `xray::Model::insert_many()`: Batch inserts proxy records
- Generated CRUD functions via `generate_model_functions!()` macro

## Implementation Details

### Import Subscription Flow

```
1. Validate URL format (must be http:// or https://)
2. Check if subscription URL already exists in database
   → If exists: Return error "Subscription URL already exists"
3. Download subscription content via HTTP request
4. Parse base64-encoded content into proxy URLs
5. Start database transaction
6. Create subscribe record
7. Parse each proxy URL and filter for xray protocols (vless/vmess/trojan)
8. Set subscribe_id on each proxy model
9. Batch insert all proxy records
10. Commit transaction
```

**Error Handling:**
- Invalid URL format → Return error immediately
- Duplicate URL → Return error before downloading
- Download failure → Propagate error to caller
- No valid proxies → Return error, rollback transaction
- Database errors → Rollback transaction via automatic drop

### Refresh Subscription Flow

```
1. Fetch subscriptions to refresh:
   - If record_ids provided: Fetch only specified subscriptions
   - If None: Fetch all subscriptions
2. For each subscription:
   a. Download new subscription content
      → On failure: Log warning, continue to next subscription
   b. Start database transaction
   c. Query all xray records linked to this subscription
   d. Delete old xray records
   e. Parse new subscription content
   f. Filter for xray protocols
   g. Batch insert new xray records
   h. Commit transaction
      → On failure: Log error, continue to next subscription
```

**Error Handling:**
- Empty subscription list → Return success (no-op)
- Download failure for a subscription → Log warning, skip that subscription
- Transaction failure → Log error, skip that subscription
- Independent processing ensures one failure doesn't affect others

### Transaction Safety

Both operations use database transactions to ensure atomicity:

**Import:**
- Single transaction creates both subscribe record and all proxy records
- On failure, entire import is rolled back
- No partial subscription data left in database

**Refresh:**
- Each subscription refreshes in its own transaction
- Delete old proxies + insert new proxies is atomic per subscription
- Failure of one subscription doesn't affect others

### Data Consistency

**Duplicate Prevention:**
- Import checks for existing URL before downloading
- No unique constraint in schema, enforced at application level
- Returns clear error message to user

**Cascade Behavior:**
- When subscription is deleted, related xray records should also be deleted
- Currently handled by finding related records via `subscribe_id`
- Future: Consider adding ON DELETE CASCADE constraint

**Orphaned Records:**
- Xray records can exist without subscription (`subscribe_id = NULL`)
- These represent manually added proxies
- Not affected by subscription operations

## Testing Considerations

**Import Function:**
- Valid HTTP subscription URL → Should create records
- Valid HTTPS subscription URL → Should create records
- Duplicate URL → Should return error without creating records
- Invalid URL format (vmess://, etc.) → Should return error
- Empty subscription → Should return error
- Network failure → Should propagate error

**Refresh Function:**
- Refresh single subscription → Should update only that subscription
- Refresh multiple subscriptions → Should update specified subscriptions
- Refresh all (None parameter) → Should update all subscriptions
- One subscription fails → Should continue processing others
- All subscriptions fail → Should return success with logged errors
- Empty subscription list → Should return success

## Future Enhancements

1. **Subscription Metadata:**
   - Add `created_at` and `updated_at` timestamps
   - Track last successful refresh time
   - Store subscription name/description

2. **Error Reporting:**
   - Return detailed results per subscription
   - Count of added/removed proxies
   - List of failed subscriptions with reasons

3. **Database Constraints:**
   - Add UNIQUE constraint on subscribe.url
   - Add ON DELETE CASCADE for xray.subscribe_id foreign key

4. **Partial Updates:**
   - Smart diff to only add/remove changed proxies
   - Preserve proxy-specific user settings (custom names, etc.)

5. **Background Refresh:**
   - Automatic periodic refresh
   - Configurable refresh intervals per subscription

6. **Import Validation:**
   - Pre-download validation of subscription content
   - Warn if subscription contains non-xray protocols

## References

- Reference implementation: XrayAPI in provided example code
- Subscription format: Base64-encoded list of proxy URLs
- Proxy URL formats: vmess://, vless://, trojan://
- Parser implementation: [parse_subscription.rs](../../src-tauri/src/apis/parse_subscription.rs)
