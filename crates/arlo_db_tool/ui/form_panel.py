import tkinter as tk
from tkinter import ttk
from typing import Any, Dict, List, Optional, Tuple, Union
import uuid
from arlo_db_tool.app_state import AppState
from arlo_db_tool.ca import (
    POSITION_CODES,
    POSITION_DISPLAY_NAMES,
    calculate_manager_ca,
    calculate_player_ca,
)
from arlo_db_tool.db.reference_lookup import get_attribute_definitions, get_options
from arlo_db_tool.schema.entities.manager import (
    ARTRINE_DEPENDENCIES,
    DEFENSIVE_APPROACHES,
    OFFENSIVE_APPROACHES,
    ROTATION_POLICIES,
)
from arlo_db_tool.schema.entities.position_codes import SHORT_POSITION_CODES
from arlo_db_tool.schema.entity_spec import CompositeEntitySpec, EntitySpec
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.ui.widgets.entity_multi_picker import EntityMultiPicker
from arlo_db_tool.ui.widgets.widget_factory import FieldWidget, create_field_widget

BUILTIN_PLAYER_ATTR_DEFS = [
    {"id": "Player:ArloControl", "key": "arlo_control", "display_name": "Arlo Control", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:Passing", "key": "passing", "display_name": "Passing", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:Dribbling", "key": "dribbling", "display_name": "Dribbling", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:Finishing", "key": "finishing", "display_name": "Finishing", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:Crossing", "key": "crossing", "display_name": "Crossing", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:OffensiveBlocking", "key": "offensive_blocking", "display_name": "Offensive Blocking", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:DefensiveContainment", "key": "defensive_containment", "display_name": "Defensive Containment", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:PasserPressure", "key": "passer_pressure", "display_name": "Passer Pressure", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:HandsReception", "key": "hands_reception", "display_name": "Hands Reception", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:DriveTechnique", "key": "drive_technique", "display_name": "Drive Technique", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:FalseArtrineBluff", "key": "false_artrine_bluff", "display_name": "False Artrine Bluff", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:Technique", "key": "technique", "display_name": "Technique", "category": "Technical", "applies_to": "Player"},
    {"id": "Player:Anticipation", "key": "anticipation", "display_name": "Anticipation", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Decisions", "key": "decisions", "display_name": "Decisions", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Composure", "key": "composure", "display_name": "Composure", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Concentration", "key": "concentration", "display_name": "Concentration", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Vision", "key": "vision", "display_name": "Vision", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Positioning", "key": "positioning", "display_name": "Positioning", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Teamwork", "key": "teamwork", "display_name": "Teamwork", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Determination", "key": "determination", "display_name": "Determination", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Leadership", "key": "leadership", "display_name": "Leadership", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Aggressiveness", "key": "aggressiveness", "display_name": "Aggressiveness", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Bravery", "key": "bravery", "display_name": "Bravery", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Flair", "key": "flair", "display_name": "Flair", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:WorkRate", "key": "work_rate", "display_name": "Work Rate", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Consistency", "key": "consistency", "display_name": "Consistency", "category": "Mental", "applies_to": "Player"},
    {"id": "Player:Acceleration", "key": "acceleration", "display_name": "Acceleration", "category": "Physical", "applies_to": "Player"},
    {"id": "Player:Pace", "key": "pace", "display_name": "Pace", "category": "Physical", "applies_to": "Player"},
    {"id": "Player:Agility", "key": "agility", "display_name": "Agility", "category": "Physical", "applies_to": "Player"},
    {"id": "Player:Balance", "key": "balance", "display_name": "Balance", "category": "Physical", "applies_to": "Player"},
    {"id": "Player:Strength", "key": "strength", "display_name": "Strength", "category": "Physical", "applies_to": "Player"},
    {"id": "Player:Stamina", "key": "stamina", "display_name": "Stamina", "category": "Physical", "applies_to": "Player"},
    {"id": "Player:JumpingReach", "key": "jumping_reach", "display_name": "Jumping Reach", "category": "Physical", "applies_to": "Player"},
    {"id": "Player:NaturalFitness", "key": "natural_fitness", "display_name": "Natural Fitness", "category": "Physical", "applies_to": "Player"},
    {"id": "Player:Reflexes", "key": "reflexes", "display_name": "Reflexes", "category": "Goalkeeping", "applies_to": "Player"},
    {"id": "Player:Handling", "key": "handling", "display_name": "Handling", "category": "Goalkeeping", "applies_to": "Player"},
    {"id": "Player:AreaCommand", "key": "area_command", "display_name": "Area Command", "category": "Goalkeeping", "applies_to": "Player"},
    {"id": "Player:Communication", "key": "communication", "display_name": "Communication", "category": "Goalkeeping", "applies_to": "Player"},
    {"id": "Player:OneOnOne", "key": "one_on_one", "display_name": "One On One", "category": "Goalkeeping", "applies_to": "Player"},
    {"id": "Player:Distribution", "key": "distribution", "display_name": "Distribution", "category": "Goalkeeping", "applies_to": "Player"},
    {"id": "Player:GoalKicking", "key": "goal_kicking", "display_name": "Goal Kicking", "category": "Goalkeeping", "applies_to": "Player"},
    {"id": "Player:RushingOut", "key": "rushing_out", "display_name": "Rushing Out", "category": "Goalkeeping", "applies_to": "Player"},
]

BUILTIN_MANAGER_ATTR_DEFS = [
    {"id": "Manager:TacticalKnowledge", "key": "tactical_knowledge", "display_name": "Tactical Knowledge", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:OffensePlanning", "key": "offense_planning", "display_name": "Offense Planning", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:DefenseOrganization", "key": "defense_organization", "display_name": "Defense Organization", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:ArtroStrategy", "key": "artro_strategy", "display_name": "Artro Strategy", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:Adaptability", "key": "adaptability", "display_name": "Adaptability", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:ArtrineCommunication", "key": "artrine_communication", "display_name": "Artrine Communication", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:TimeCallManagement", "key": "time_call_management", "display_name": "Time Call Management", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:ChallengeJudgment", "key": "challenge_judgment", "display_name": "Challenge Judgment", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:InGameAdjustments", "key": "in_game_adjustments", "display_name": "In Game Adjustments", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:ManManagement", "key": "man_management", "display_name": "Man Management", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:LoadManagement", "key": "load_management", "display_name": "Load Management", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:PlayerDevelopment", "key": "player_development", "display_name": "Player Development", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:JudgingAbility", "key": "judging_ability", "display_name": "Judging Ability", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:JudgingPotential", "key": "judging_potential", "display_name": "Judging Potential", "category": "Managerial", "applies_to": "Manager"},
    {"id": "Manager:Discipline", "key": "discipline", "display_name": "Discipline", "category": "Managerial", "applies_to": "Manager"},
]

class FormPanel(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent, padding=(8, 8))
        self.app_state = app_state
        self.field_widgets: Dict[str, FieldWidget] = {}
        self.attribute_widgets: Dict[str, Dict[str, Any]] = {}
        self.position_widgets: Dict[str, Dict[str, Any]] = {}
        self.tp_vars: Dict[str, tk.StringVar] = {}
        self.tp_widgets: List[Any] = []
        self.tp_enabled_var = tk.BooleanVar(value=False)
        self.preferred_formations_picker: Optional[EntityMultiPicker] = None
        self.current_spec: Optional[Union[EntitySpec, CompositeEntitySpec]] = None
        self.lbl_ca_value: Optional[ttk.Label] = None
        self.ca_progress: Optional[ttk.Progressbar] = None
        self.ca_position_var: Optional[tk.StringVar] = None
        self.ca_position_combo: Optional[ttk.Combobox] = None

        self.canvas = tk.Canvas(self, borderwidth=0, highlightthickness=0)
        self.scrollbar = ttk.Scrollbar(self, orient=tk.VERTICAL, command=self.canvas.yview)
        self.scroll_content = ttk.Frame(self.canvas)

        self.scroll_content.bind(
            "<Configure>",
            lambda _e: self.canvas.configure(scrollregion=self.canvas.bbox("all")),
        )
        self.canvas_frame = self.canvas.create_window((0, 0), window=self.scroll_content, anchor="nw")
        self.canvas.configure(xscrollcommand=None, yscrollcommand=self.scrollbar.set)

        self.canvas.bind(
            "<Configure>",
            lambda e: self.canvas.itemconfig(self.canvas_frame, width=e.width),
        )

        self.canvas.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        self.scrollbar.pack(side=tk.RIGHT, fill=tk.Y)

    def build_form(self, spec: Union[EntitySpec, CompositeEntitySpec]):
        self.current_spec = spec
        self.field_widgets.clear()
        self.attribute_widgets.clear()
        self.position_widgets.clear()
        self.tp_vars.clear()
        self.tp_widgets.clear()
        self.preferred_formations_picker = None
        self.lbl_ca_value = None
        self.ca_progress = None
        self.ca_position_var = None
        self.ca_position_combo = None

        for child in self.scroll_content.winfo_children():
            child.destroy()

        self.scroll_content.columnconfigure(0, weight=0)
        self.scroll_content.columnconfigure(1, weight=1)

        fields = (
            spec.get_all_fields()
            if isinstance(spec, CompositeEntitySpec)
            else spec.fields
        )
        current_row = 0
        for row_idx, field in enumerate(fields):
            req_marker = " *" if not field.nullable else ""
            lbl_text = f"{field.name}{req_marker}:"
            lbl = ttk.Label(self.scroll_content, text=lbl_text, anchor="w")
            lbl.grid(row=row_idx, column=0, sticky="w", padx=(4, 12), pady=6)

            fw = create_field_widget(self.scroll_content, field, self.app_state)
            fw.container.grid(row=row_idx, column=1, sticky="ew", padx=(0, 4), pady=6)
            self.field_widgets[field.name] = fw
            current_row = row_idx + 1

        if spec.name == "Player":
            current_row = self._build_positions_section(current_row)

        if spec.name == "Manager":
            current_row = self._build_tactical_profile_section(current_row)

        if spec.name in ("Player", "Manager"):
            self._build_attributes_section(spec.name, current_row)

    def _build_positions_section(self, start_row: int) -> int:
        sep = ttk.Separator(self.scroll_content, orient=tk.HORIZONTAL)
        sep.grid(row=start_row, column=0, columnspan=2, sticky="ew", pady=(16, 8))

        pos_frame = ttk.LabelFrame(self.scroll_content, text="Posições e Proficiência (1 a 10)", padding=(12, 10))
        pos_frame.grid(row=start_row + 1, column=0, columnspan=2, sticky="ew", padx=4, pady=(0, 8))

        for i in range(3):
            pos_frame.columnconfigure(i * 2, weight=1)

        for i, code in enumerate(SHORT_POSITION_CODES):
            name = POSITION_DISPLAY_NAMES.get(code, code)
            row = i // 3
            col = (i % 3) * 2

            chk_var = tk.BooleanVar(value=False)
            prof_var = tk.StringVar(value="10")

            def toggle_spin(c=code):
                info = self.position_widgets[c]
                if info["chk_var"].get():
                    info["spin"].config(state="normal")
                else:
                    info["spin"].config(state="disabled")

            chk = ttk.Checkbutton(pos_frame, text=name, variable=chk_var, command=toggle_spin)
            chk.grid(row=row, column=col, sticky="w", padx=(4, 2), pady=4)

            spin = ttk.Spinbox(pos_frame, from_=1, to=10, increment=1, textvariable=prof_var, width=3, state="disabled")
            spin.grid(row=row, column=col + 1, sticky="w", padx=(0, 16), pady=4)

            self.position_widgets[code] = {
                "chk_var": chk_var,
                "prof_var": prof_var,
                "spin": spin,
                "chk": chk,
            }

        return start_row + 2

    def _build_tactical_profile_section(self, start_row: int) -> int:
        sep = ttk.Separator(self.scroll_content, orient=tk.HORIZONTAL)
        sep.grid(row=start_row, column=0, columnspan=2, sticky="ew", pady=(16, 8))

        tp_frame = ttk.LabelFrame(self.scroll_content, text="Perfil Tático (Opcional)", padding=(12, 10))
        tp_frame.grid(row=start_row + 1, column=0, columnspan=2, sticky="ew", padx=4, pady=(0, 8))

        self.tp_enabled_var = tk.BooleanVar(value=False)
        chk_enable = ttk.Checkbutton(tp_frame, text="Habilitar Perfil Tático", variable=self.tp_enabled_var, command=self._update_tp_state)
        chk_enable.grid(row=0, column=0, columnspan=4, sticky="w", pady=(0, 8))

        self.tp_vars = {}
        self.tp_widgets = []

        enum_fields = [
            ("Abordagem Ofensiva", "offensive_approach", OFFENSIVE_APPROACHES),
            ("Abordagem Defensiva", "defensive_approach", DEFENSIVE_APPROACHES),
            ("Política de Rotação", "rotation_policy", ROTATION_POLICIES),
            ("Dependência do Artrine", "artrine_dependency", ARTRINE_DEPENDENCIES),
        ]
        
        for i, (label, key, values) in enumerate(enum_fields):
            r = (i // 2) + 1
            c = (i % 2) * 2
            ttk.Label(tp_frame, text=f"{label}:").grid(row=r, column=c, sticky="w", padx=(0, 4), pady=4)
            var = tk.StringVar(value=values[0])
            self.tp_vars[key] = var
            cb = ttk.Combobox(tp_frame, textvariable=var, values=values, state="readonly", width=16)
            cb.grid(row=r, column=c+1, sticky="w", padx=(0, 16), pady=4)
            self.tp_widgets.append(cb)

        float_fields = [
            ("Tendência de Flexibilidade (0 a 1)", "flexibility_tendency", 0.0, 1.0, 0.5),
            ("Pref. Raio de Passe (-1 a 1)", "passing_range_preference", -1.0, 1.0, 0.0),
            ("Pref. Jogo Aéreo (-1 a 1)", "aeriality_preference", -1.0, 1.0, 0.0),
            ("Pref. Estrutura (-1 a 1)", "structure_preference", -1.0, 1.0, 0.0),
            ("Pref. Fisicalidade (0 a 1)", "physicality_preference", 0.0, 1.0, 0.5),
            ("Pref. Ritmo Transição (0 a 1)", "transition_pace_preference", 0.0, 1.0, 0.5),
            ("Pref. Formato Bloco Pressão (0 a 1)", "press_block_shape_preference", 0.0, 1.0, 0.5),
        ]
        
        for i, (label, key, f_min, f_max, f_def) in enumerate(float_fields):
            r = (i // 2) + 3
            c = (i % 2) * 2
            ttk.Label(tp_frame, text=f"{label}:").grid(row=r, column=c, sticky="w", padx=(0, 4), pady=4)
            var = tk.StringVar(value=str(f_def))
            self.tp_vars[key] = var
            spin = ttk.Spinbox(tp_frame, from_=f_min, to=f_max, increment=0.05, textvariable=var, width=6)
            spin.grid(row=r, column=c+1, sticky="w", padx=(0, 16), pady=4)
            self.tp_widgets.append(spin)

        pf_frame = ttk.LabelFrame(tp_frame, text="Formações Preferidas (Máx 3)", padding=(8, 6))
        pf_frame.grid(row=8, column=0, columnspan=4, sticky="ew", pady=(8, 0))
        
        self.preferred_formations_picker = EntityMultiPicker(pf_frame, left_title="Formações Disponíveis", right_title="Selecionadas", height=4)
        self.preferred_formations_picker.pack(fill=tk.BOTH, expand=True)
        self.tp_widgets.append(self.preferred_formations_picker)

        if self.app_state.database_path:
            formations = get_options(self.app_state.database_path, "formations", id_col="id", label_expr="name")
            self.preferred_formations_picker.set_available_items(formations)

        self._update_tp_state()
        return start_row + 2

    def _update_tp_state(self):
        state = "readonly" if self.tp_enabled_var.get() else "disabled"
        spin_state = "normal" if self.tp_enabled_var.get() else "disabled"
        for w in self.tp_widgets:
            if isinstance(w, ttk.Combobox):
                w.config(state=state)
            elif isinstance(w, ttk.Spinbox):
                w.config(state=spin_state)
            elif isinstance(w, EntityMultiPicker):
                w.set_enabled(self.tp_enabled_var.get())

    def _build_attributes_section(self, entity_name: str, start_row: int):
        target = "Player" if entity_name == "Player" else "Manager"

        sep = ttk.Separator(self.scroll_content, orient=tk.HORIZONTAL)
        sep.grid(row=start_row, column=0, columnspan=2, sticky="ew", pady=(16, 8))

        ca_frame = ttk.LabelFrame(self.scroll_content, text="Current Ability (CA) • Escala 1 - 200", padding=(12, 10))
        ca_frame.grid(row=start_row + 1, column=0, columnspan=2, sticky="ew", padx=4, pady=(0, 8))
        ca_frame.columnconfigure(0, weight=1)
        ca_frame.columnconfigure(1, weight=1)

        left_info = ttk.Frame(ca_frame)
        left_info.grid(row=0, column=0, sticky="w", padx=(0, 16))

        if entity_name == "Player":
            pos_lbl = ttk.Label(left_info, text="Posição de Referência para CA:")
            pos_lbl.pack(anchor="w", pady=(0, 2))

            display_options = [POSITION_DISPLAY_NAMES[code] for code in POSITION_CODES]
            self.ca_position_var = tk.StringVar(value=POSITION_DISPLAY_NAMES["P"])
            self.ca_position_combo = ttk.Combobox(
                left_info,
                textvariable=self.ca_position_var,
                values=display_options,
                state="readonly",
                width=28,
            )
            self.ca_position_combo.pack(anchor="w")
            self.ca_position_combo.bind("<<ComboboxSelected>>", lambda _e: self._update_ca_display())
        else:
            mgr_lbl = ttk.Label(left_info, text="Capacidade Tática e Gestão Geral do Manager")
            mgr_lbl.pack(anchor="w", pady=(4, 2))

        right_display = ttk.Frame(ca_frame)
        right_display.grid(row=0, column=1, sticky="e")

        val_row = ttk.Frame(right_display)
        val_row.pack(anchor="e")

        ca_title = ttk.Label(val_row, text="CA Estimado: ", font=("TkDefaultFont", 11, "bold"))
        ca_title.pack(side=tk.LEFT)

        self.lbl_ca_value = ttk.Label(
            val_row,
            text="100",
            font=("TkDefaultFont", 16, "bold"),
            foreground="#005fb8",
        )
        self.lbl_ca_value.pack(side=tk.LEFT)

        ca_max_lbl = ttk.Label(val_row, text=" / 200", font=("TkDefaultFont", 10))
        ca_max_lbl.pack(side=tk.LEFT)

        self.ca_progress = ttk.Progressbar(
            right_display,
            orient=tk.HORIZONTAL,
            length=200,
            maximum=200,
            value=100,
        )
        self.ca_progress.pack(anchor="e", pady=(4, 0))

        hdr_frame = ttk.Frame(self.scroll_content)
        hdr_frame.grid(row=start_row + 2, column=0, columnspan=2, sticky="ew", padx=4, pady=(6, 6))

        hdr_label = ttk.Label(
            hdr_frame,
            text=f"Atributos de {entity_name} (Valores de 1 a 20)",
            font=("TkDefaultFont", 10, "bold"),
        )
        hdr_label.pack(side=tk.LEFT)

        defs = get_attribute_definitions(self.app_state.database_path, target)
        if not defs:
            defs = BUILTIN_PLAYER_ATTR_DEFS if target == "Player" else BUILTIN_MANAGER_ATTR_DEFS

        categories: Dict[str, List[Dict[str, Any]]] = {}
        for attr_def in defs:
            cat = attr_def.get("category") or "Geral"
            categories.setdefault(cat, []).append(attr_def)

        grid_row = start_row + 3
        for cat_name, cat_defs in categories.items():
            cat_frame = ttk.LabelFrame(self.scroll_content, text=cat_name, padding=(8, 6))
            cat_frame.grid(row=grid_row, column=0, columnspan=2, sticky="ew", padx=4, pady=4)
            cat_frame.columnconfigure(1, weight=1)
            cat_frame.columnconfigure(3, weight=1)

            for i, attr_def in enumerate(cat_defs):
                r = i // 2
                c_base = (i % 2) * 2

                disp_name = attr_def.get("display_name") or attr_def.get("key", "")
                attr_id = str(attr_def["id"])

                attr_lbl = ttk.Label(cat_frame, text=f"{disp_name}:", anchor="w")
                attr_lbl.grid(row=r, column=c_base, sticky="w", padx=(6, 4), pady=3)

                attr_var = tk.StringVar(value="10")
                attr_var.trace_add("write", lambda *_: self._update_ca_display())

                spin = ttk.Spinbox(
                    cat_frame,
                    from_=1,
                    to=20,
                    increment=1,
                    textvariable=attr_var,
                    width=6,
                )
                spin.grid(row=r, column=c_base + 1, sticky="w", padx=(0, 16), pady=3)

                self.attribute_widgets[attr_id] = {
                    "var": attr_var,
                    "widget": spin,
                    "def": attr_def,
                }

            grid_row += 1

        self._update_ca_display()

    def _extract_position_code(self) -> str:
        if not self.ca_position_var:
            return "P"
        raw = self.ca_position_var.get().strip()
        if "(" in raw and ")" in raw:
            return raw.split("(")[-1].split(")")[0].strip()
        for code, name in POSITION_DISPLAY_NAMES.items():
            if name == raw or code == raw:
                return code
        return "P"

    def _update_ca_display(self):
        if not self.current_spec or self.current_spec.name not in ("Player", "Manager"):
            return
        if self.lbl_ca_value is None:
            return

        attr_dict: Dict[str, float] = {}
        for _attr_id, info in self.attribute_widgets.items():
            attr_def = info.get("def", {})
            key = attr_def.get("key")
            if not key:
                continue
            raw_val = info["var"].get().strip()
            try:
                val = float(raw_val)
            except (ValueError, TypeError):
                val = 10.0
            attr_dict[key] = val

        if self.current_spec.name == "Player":
            pos_code = self._extract_position_code()
            ca = calculate_player_ca(attr_dict, pos_code)
        else:
            ca = calculate_manager_ca(attr_dict)

        self.lbl_ca_value.config(text=str(ca))
        if self.ca_progress is not None:
            self.ca_progress["value"] = ca

    def get_values(self) -> Dict[str, Any]:
        result = {}
        for name, fw in self.field_widgets.items():
            result[name] = fw.get_value()
        return result

    def set_values(self, values: Dict[str, Any]):
        for name, fw in self.field_widgets.items():
            if name in values:
                fw.set_value(values[name])
            elif self.current_spec:
                field = self.current_spec.get_field(name)
                if field and field.column in values:
                    fw.set_value(values[field.column])

    def get_tactical_profile_values(self) -> Tuple[bool, Dict[str, Any], List[str]]:
        enabled = self.tp_enabled_var.get()
        values = {}
        for f in ["offensive_approach", "defensive_approach", "rotation_policy", "artrine_dependency"]:
            if f in self.tp_vars:
                values[f] = self.tp_vars[f].get()
        for f in ["flexibility_tendency", "passing_range_preference", "aeriality_preference", "structure_preference", "physicality_preference", "transition_pace_preference", "press_block_shape_preference"]:
            if f in self.tp_vars:
                try:
                    values[f] = float(self.tp_vars[f].get())
                except ValueError:
                    values[f] = 0.0
        pf_values = self.preferred_formations_picker.get_selected_ids() if self.preferred_formations_picker else []
        return enabled, values, pf_values

    def set_tactical_profile_values(self, tp_data: Dict[str, Any], pf_data: List[str]):
        self.tp_enabled_var.set(True)
        for f in ["offensive_approach", "defensive_approach", "rotation_policy", "artrine_dependency"]:
            if f in self.tp_vars and f in tp_data:
                self.tp_vars[f].set(str(tp_data[f]))
        for f in ["flexibility_tendency", "passing_range_preference", "aeriality_preference", "structure_preference", "physicality_preference", "transition_pace_preference", "press_block_shape_preference"]:
            if f in self.tp_vars and f in tp_data:
                self.tp_vars[f].set(str(tp_data[f]))
        if self.preferred_formations_picker:
            self.preferred_formations_picker.set_selected_ids(pf_data)
        self._update_tp_state()

    def reset_tactical_profile_values(self):
        self.tp_enabled_var.set(False)
        if "offensive_approach" in self.tp_vars:
            self.tp_vars["offensive_approach"].set("Balanced")
        if "defensive_approach" in self.tp_vars:
            self.tp_vars["defensive_approach"].set("MidBlock")
        if "rotation_policy" in self.tp_vars:
            self.tp_vars["rotation_policy"].set("Situational")
        if "artrine_dependency" in self.tp_vars:
            self.tp_vars["artrine_dependency"].set("SystemDriven")
        for f in ["flexibility_tendency", "physicality_preference", "transition_pace_preference", "press_block_shape_preference"]:
            if f in self.tp_vars:
                self.tp_vars[f].set("0.5")
        for f in ["passing_range_preference", "aeriality_preference", "structure_preference"]:
            if f in self.tp_vars:
                self.tp_vars[f].set("0.0")
        if self.preferred_formations_picker:
            self.preferred_formations_picker.clear()
        self._update_tp_state()

    def get_attribute_values(self) -> Dict[str, Any]:
        result = {}
        for attr_id, info in self.attribute_widgets.items():
            result[attr_id] = info["var"].get().strip()
        return result

    def set_attribute_values(self, values: Dict[str, Any]):
        for attr_id, info in self.attribute_widgets.items():
            if attr_id in values:
                info["var"].set(str(values[attr_id]))
            else:
                attr_def = info.get("def", {})
                key = attr_def.get("key")
                if key and key in values:
                    info["var"].set(str(values[key]))
                else:
                    info["var"].set("10")
        self._update_ca_display()

    def reset_attribute_values(self):
        for info in self.attribute_widgets.values():
            info["var"].set("10")
        self._update_ca_display()

    def get_position_values(self) -> Dict[str, int]:
        result = {}
        for code, info in self.position_widgets.items():
            if info["chk_var"].get():
                try:
                    val = int(info["prof_var"].get())
                    result[code] = max(1, min(10, val))
                except (ValueError, TypeError):
                    result[code] = 10
        return result

    def set_position_values(self, values: Dict[str, int]):
        for code, info in self.position_widgets.items():
            if code in values:
                info["chk_var"].set(True)
                info["prof_var"].set(str(values[code]))
                info["spin"].config(state="normal")
            else:
                info["chk_var"].set(False)
                info["prof_var"].set("10")
                info["spin"].config(state="disabled")

    def reset_position_values(self):
        for code, info in self.position_widgets.items():
            info["chk_var"].set(False)
            info["prof_var"].set("10")
            info["spin"].config(state="disabled")

    def reset_values(self):
        if not self.current_spec:
            return
        fields = (
            self.current_spec.get_all_fields()
            if isinstance(self.current_spec, CompositeEntitySpec)
            else self.current_spec.fields
        )
        for field in fields:
            fw = self.field_widgets.get(field.name)
            if not fw:
                continue
            if field.primary_key:
                if field.field_type == FieldType.UUID_PK:
                    fw.set_value(str(uuid.uuid4()))
                else:
                    fw.reload()
            else:
                fw.set_value(field.default)
        self.reset_attribute_values()
        self.reset_position_values()
        self.reset_tactical_profile_values()

    def reload_references(self):
        for fw in self.field_widgets.values():
            fw.reload()
        if self.preferred_formations_picker:
            formations = get_options(self.app_state.database_path, "formations", id_col="id", label_expr="name")
            self.preferred_formations_picker.set_available_items(formations)
        if self.current_spec and self.current_spec.name in ("Player", "Manager"):
            saved_main = self.get_values()
            saved_attrs = self.get_attribute_values()
            saved_positions = self.get_position_values()
            en, tp, pf = self.get_tactical_profile_values()
            self.build_form(self.current_spec)
            self.set_values(saved_main)
            self.set_attribute_values(saved_attrs)
            self.set_position_values(saved_positions)
            if en:
                self.set_tactical_profile_values(tp, pf)
            else:
                self.reset_tactical_profile_values()
            if self.preferred_formations_picker:
                formations = get_options(self.app_state.database_path, "formations", id_col="id", label_expr="name")
                self.preferred_formations_picker.set_available_items(formations)

    def set_enabled(self, enabled: bool):
        for fw in self.field_widgets.values():
            fw.set_enabled(enabled)
        for info in self.attribute_widgets.values():
            info["widget"].config(state="normal" if enabled else "disabled")
        for info in self.position_widgets.values():
            info["chk"].config(state="normal" if enabled else "disabled")
            if enabled and info["chk_var"].get():
                info["spin"].config(state="normal")
            else:
                info["spin"].config(state="disabled")
        if self.ca_position_combo is not None:
            self.ca_position_combo.config(state="readonly" if enabled else "disabled")
        if self.current_spec and self.current_spec.name == "Manager":
            self.tp_enabled_var.set(enabled and self.tp_enabled_var.get())
            self._update_tp_state()