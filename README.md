# maan-backend
## API Documentation

### 1. Create Beneficiary (UL)

**Endpoint:** `/create_beneficiary_ul`  
**Method:** `POST`  
**Description:** Creates a new ul beneficiary.

**Request JSON:**
```json
{
  "inn": "string",
  "nominal_account_code": "string",
  "nominal_account_bic": "string",
  "beneficiary_data": {
    "name": "string",
    "kpp": "string",
    "ogrn": "string",
  }
}
```

**Response JSON:**
```json
{
    "beneficiary": {
        "inn": "string",
        "nominal_account_code": "string",
        "nominal_account_bic": "string",
        "id": "string",
    }
}
```

### 2. List Beneficiary

**Endpoint:** `/list_beneficiary`  
**Method:** `GET`  
**Description:** Retrieves a list of all beneficiaries.

**Request JSON**
```json
{
  "filters": {
    "inn": "string",
    "nominal_account_code": "string",
    "nominal_account_bic": "string",
    "is_active": "boolean",
    "legal_type": "string"
  }
}
```

**Response JSON:**
```json
{
  "beneficiaries": [
    {
      "id": "string",
      "inn": "string",
      "nominal_account_code": "string",
      "nominal_account_bic": "string",
      "is_active": "boolean",
      "legal_type": "string"
    }
  ]
}
```

### 3. Get Beneficiary

**Endpoint:** `/get_beneficiary`  
**Method:** `GET`  
**Description:** Retrieves details of a specific beneficiary.

**Request JSON**
```json
{
  "beneficiary_id": "string"
}
```

**Response JSON:**
```json
{
  "beneficiary": {
    "id": "string",
    "inn": "string",
    "is_active": "boolean",
    "is_added_to_ms": "boolean",
    "legal_type": "string",
    "ogrn": "string",
    "beneficiary_data": {
      "name": "string",
      "kpp": "string",
      "ogrn": "string",
      "is_branch": "boolean"
    },
    "created_at": "string",
    "updated_at": "string"
  },
  "nominal_account": {
    "code": "string",
    "bic": "string"
  },
  "last_contract_offer": "object",
  "permission": "boolean",
  "permission_description": "string"
}
```

### 4. Create Charity Project

**Endpoint:** `/create_charity_project`  
**Method:** `POST`  
**Description:** Creates a new charity project.

**Request JSON:**
```json
{
  "beneficiary_id": "string",
  "name": "string",
  "description": "string"
}
```

**Response JSON:**
Returns id of the created project (actually, virtual accound id).
```json
{
  "id": "string"
}
```

### 5. List All Projects

**Endpoint:** `/list_all_projects`  
**Method:** `GET`  
**Description:** Retrieves a list of all charity projects.

**Response JSON:**
```json
{
    "projects": [
        {
            "name": "string",
            "description": "string",
            "id": "string",
        }
    ]
}
```

### 6. List Beneficiary Projects

**Endpoint:** `/list_beneficiary_projects`  
**Method:** `GET`  
**Description:** Retrieves a list of all projects for a specific beneficiary.

**Request JSON:**
```json
{
  "beneficiary_id": "string",
}
```

**Response JSON:**
```json
{
    "projects": [
        {
            "project_name": "string",
            "description": "string",
            "id": "string",
        }
    ]
}
```

### 7. Get Charity Project

**Endpoint:** `/get_charity_project`  
**Method:** `GET`  
**Description:** Retrieves details of a specific charity project.

**Request JSON:**
```json
{
  "project_id": "string"
}
```

**Response JSON:**
```json
{
    "project": {
        "name": "string",
        "description": "string",
        "id": "string",
    }
}
```

### 8. Upload Document Beneficiary
**Endpoint:** `/upload_document_beneficiary`
**Method:** `POST`
**Description:** Uploads a document for a beneficiary.

**Request JSON:**
```json
{
  "b64_document": "string",
  "beneficiary_id": "string",
  "document_number": "string",
  "document_date": "string",
  "content_type": "string"
}
```

**Response JSON:**
```json
{
  "document_id": "string"
}
```

### 8. Get Document
**Endpoint:** `/get_document`
**Method:** `GET`
**Description:**  Retrieves a specific document by ID.

**Request JSON:**
```json
{
  "document_id": "string"
}
```

**Response JSON:**
```json
{
  "id": "string",
  "type": "string",
  "document_number": "string",
  "document_date": "string",
  "success_added": "boolean",
  "success_added_desc": "string",
  "deal_id": null || "string"
}
```
