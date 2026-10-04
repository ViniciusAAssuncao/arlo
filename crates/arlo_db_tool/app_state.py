from typing import Any, Dict, List, Optional

class AppState:
    def __init__(self):
        self.database_path: Optional[str] = None
        self.current_entity: Optional[str] = None
        self.current_operation: str = "Create"
        self.selected_record_id: Optional[str] = None
        self.original_record_data: Dict[str, Any] = {}
        self.original_attribute_data: Dict[str, Dict[str, Any]] = {}
        self.original_position_data: Dict[str, int] = {}
        self.original_tactical_profile_data: Optional[Dict[str, Any]] = None
        self.original_preferred_formations: List[str] = []
        self.form_data: Dict[str, Any] = {}
        self.is_dirty: bool = False