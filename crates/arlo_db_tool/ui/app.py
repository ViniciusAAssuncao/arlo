import tkinter as tk
from tkinter import messagebox, ttk
from typing import Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.reference_lookup import get_entity_attributes, get_manager_preferred_formations, get_manager_tactical_profile, get_player_positions, get_record_by_id
from arlo_db_tool.schema.entity_spec import CompositeEntitySpec
from arlo_db_tool.schema.registry import get_entity_spec
from arlo_db_tool.sql.statement_builder import (
    build_attribute_delete_statements,
    build_attribute_insert_statements,
    build_attribute_update_statements,
    build_composite_delete_statements,
    build_composite_insert_statements,
    build_composite_update_statements,
    build_delete_statement,
    build_insert_statement,
    build_manager_tactical_profile_delete_statements,
    build_manager_tactical_profile_insert_statements,
    build_manager_tactical_profile_update_statements,
    build_player_position_delete_statements,
    build_player_position_insert_statements,
    build_player_position_update_statements,
    build_update_statement,
)
from arlo_db_tool.ui.bulk.bulk_hub_window import BulkHubWindow
from arlo_db_tool.ui.db_path_bar import DbPathBar
from arlo_db_tool.ui.entity_selector import EntitySelector
from arlo_db_tool.ui.form_panel import FormPanel
from arlo_db_tool.ui.league_calendar_config_screen import LeagueCalendarConfigScreen
from arlo_db_tool.ui.operation_selector import OperationSelector
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel
from arlo_db_tool.validation.validators import validate_field_spec


class App(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Arlo Database Tool")
        self.geometry("1024x768")
        self.minsize(800, 600)

        self.app_state = AppState()
        self.config_window: Optional[tk.Toplevel] = None
        self.config_screen: Optional[LeagueCalendarConfigScreen] = None
        self.bulk_hub_window: Optional[BulkHubWindow] = None

        self.db_bar = DbPathBar(
            self,
            self.app_state,
            on_db_changed=self._on_db_changed,
            on_reload_references=self._on_reload_references,
        )
        self.db_bar.pack(fill=tk.X)

        sep1 = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep1.pack(fill=tk.X)

        top_controls = ttk.Frame(self)
        top_controls.pack(fill=tk.X)

        self.entity_selector = EntitySelector(
            top_controls,
            self.app_state,
            on_entity_changed=self._on_entity_changed,
        )
        self.entity_selector.pack(side=tk.LEFT)

        self.op_selector = OperationSelector(
            top_controls,
            self.app_state,
            on_operation_changed=self._on_operation_changed,
            on_target_record_changed=self._on_target_record_changed,
        )
        self.op_selector.pack(side=tk.LEFT, fill=tk.X, expand=True)

        self.btn_bulk_tools = ttk.Button(
            top_controls,
            text="Ferramentas em Massa",
            command=self._open_bulk_hub,
        )
        self.btn_bulk_tools.pack(side=tk.RIGHT, padx=4)

        self.btn_open_config = ttk.Button(
            top_controls,
            text="Configurador de Calendário de Liga",
            command=self._open_league_calendar_config,
        )
        self.btn_open_config.pack(side=tk.RIGHT, padx=4)

        sep2 = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep2.pack(fill=tk.X)

        self.form_panel = FormPanel(self, self.app_state)
        self.form_panel.pack(fill=tk.BOTH, expand=True)

        action_bar = ttk.Frame(self, padding=(8, 4))
        action_bar.pack(fill=tk.X)

        self.btn_generate = ttk.Button(
            action_bar,
            text="Gerar SQL",
            command=self._generate_sql,
        )
        self.btn_generate.pack(side=tk.RIGHT, padx=4)

        sep3 = ttk.Separator(self, orient=tk.HORIZONTAL)
        sep3.pack(fill=tk.X)

        self.sql_output = SqlOutputPanel(self)
        self.sql_output.pack(fill=tk.X)

        initial_entity = self.entity_selector.get_selected_entity()
        if initial_entity:
            self._on_entity_changed(initial_entity)

    def _open_league_calendar_config(self):
        if self.config_window is not None and self.config_window.winfo_exists():
            self.config_window.lift()
            return

        self.config_window = tk.Toplevel(self)
        self.config_window.title(
            "Configurador de Calendário de Liga (League Calendar Config)")
        self.config_window.geometry("1080x850")
        self.config_window.minsize(800, 600)

        self.config_screen = LeagueCalendarConfigScreen(
            self.config_window, self.app_state)
        self.config_screen.pack(fill=tk.BOTH, expand=True)

    def _open_bulk_hub(self):
        if self.bulk_hub_window is not None and self.bulk_hub_window.winfo_exists():
            self.bulk_hub_window.lift()
            return

        self.bulk_hub_window = BulkHubWindow(self, self.app_state)

    def _on_db_changed(self, _db_path: str):
        self.form_panel.reload_references()
        self.op_selector.reload_target_records()
        if self.config_screen is not None and self.config_window is not None and self.config_window.winfo_exists():
            self.config_screen.reload_references()
        if self.bulk_hub_window is not None and self.bulk_hub_window.winfo_exists():
            self.bulk_hub_window.reload_references()

    def _on_reload_references(self):
        self.form_panel.reload_references()
        self.op_selector.reload_target_records()
        if self.config_screen is not None and self.config_window is not None and self.config_window.winfo_exists():
            self.config_screen.reload_references()
        if self.bulk_hub_window is not None and self.bulk_hub_window.winfo_exists():
            self.bulk_hub_window.reload_references()

    def _on_entity_changed(self, entity_name: str):
        spec = get_entity_spec(entity_name)
        if spec:
            self.form_panel.build_form(spec)
            self.op_selector.reload_target_records()
            self._apply_operation_state()

    def _on_operation_changed(self, _operation: str):
        self._apply_operation_state()

    def _on_target_record_changed(self, record_id: Optional[str]):
        op = self.op_selector.get_selected_operation()
        if op == "Update" and record_id:
            entity_name = self.app_state.current_entity
            if not entity_name or not self.app_state.database_path:
                return
            spec = get_entity_spec(entity_name)
            if not spec:
                return
            if isinstance(spec, CompositeEntitySpec):
                combined = {}
                for sub_spec in spec.specs:
                    pk_field = sub_spec.primary_key_field()
                    pk_col = pk_field.column if pk_field else "id"
                    rec = get_record_by_id(
                        self.app_state.database_path, sub_spec.table_name, pk_col, record_id)
                    if rec:
                        combined.update(rec)
                if combined:
                    self.app_state.original_record_data = dict(combined)
                    self.form_panel.set_values(combined)
            else:
                pk_field = spec.primary_key_field()
                pk_col = pk_field.column if pk_field else "id"
                record = get_record_by_id(
                    self.app_state.database_path, spec.table_name, pk_col, record_id)
                if record:
                    self.app_state.original_record_data = dict(record)
                    self.form_panel.set_values(record)

            if entity_name == "Player":
                attr_data = get_entity_attributes(
                    self.app_state.database_path,
                    "player_attributes",
                    "player_id",
                    record_id,
                )
                self.app_state.original_attribute_data = attr_data
                self.form_panel.set_attribute_values(
                    {k: v["value"] for k, v in attr_data.items()})

                pos_data = get_player_positions(
                    self.app_state.database_path, record_id)
                self.app_state.original_position_data = pos_data
                self.form_panel.set_position_values(pos_data)

            elif entity_name == "Manager":
                attr_data = get_entity_attributes(
                    self.app_state.database_path,
                    "manager_attributes",
                    "manager_id",
                    record_id,
                )
                self.app_state.original_attribute_data = attr_data
                self.form_panel.set_attribute_values(
                    {k: v["value"] for k, v in attr_data.items()})
                
                tp_data = get_manager_tactical_profile(self.app_state.database_path, record_id)
                if tp_data:
                    self.app_state.original_tactical_profile_data = tp_data
                    pf_data = get_manager_preferred_formations(self.app_state.database_path, tp_data["id"])
                    self.app_state.original_preferred_formations = pf_data
                    self.form_panel.set_tactical_profile_values(tp_data, pf_data)
                else:
                    self.app_state.original_tactical_profile_data = None
                    self.app_state.original_preferred_formations = []
                    self.form_panel.reset_tactical_profile_values()
            else:
                self.app_state.original_attribute_data = {}
                self.app_state.original_position_data = {}
                self.app_state.original_tactical_profile_data = None
                self.app_state.original_preferred_formations = []

    def _apply_operation_state(self):
        op = self.op_selector.get_selected_operation()
        if op == "Create":
            self.form_panel.set_enabled(True)
            self.form_panel.reset_values()
            self.app_state.original_record_data = {}
            self.app_state.original_attribute_data = {}
            self.app_state.original_position_data = {}
            self.app_state.original_tactical_profile_data = None
            self.app_state.original_preferred_formations = []
        elif op == "Update":
            self.form_panel.set_enabled(True)
            target_id = self.op_selector.get_selected_target_id()
            if target_id:
                self._on_target_record_changed(target_id)
            else:
                self.form_panel.reset_values()
                self.app_state.original_record_data = {}
                self.app_state.original_attribute_data = {}
                self.app_state.original_position_data = {}
                self.app_state.original_tactical_profile_data = None
                self.app_state.original_preferred_formations = []
        elif op == "Delete":
            self.form_panel.set_enabled(False)

    def _generate_sql(self):
        entity_name = self.app_state.current_entity
        if not entity_name:
            messagebox.showerror("Erro", "Nenhuma entidade selecionada.")
            return

        spec = get_entity_spec(entity_name)
        if not spec:
            messagebox.showerror(
                "Erro", f"Especificação não encontrada para a entidade '{entity_name}'.")
            return

        op = self.op_selector.get_selected_operation()

        attr_values = {}
        if entity_name in ("Player", "Manager") and op in ("Create", "Update"):
            attr_values = self.form_panel.get_attribute_values()
            attr_errors = []
            for def_id, raw_val in attr_values.items():
                if raw_val is None or str(raw_val).strip() == "":
                    continue
                try:
                    val = int(raw_val)
                    if val < 1 or val > 20:
                        def_info = self.form_panel.attribute_widgets.get(
                            def_id, {})
                        def_dict = def_info.get("def", {})
                        name = def_dict.get(
                            "display_name") or def_dict.get("key") or def_id
                        attr_errors.append(
                            f"Atributo '{name}' deve ser um número inteiro entre 1 e 20.")
                except (ValueError, TypeError):
                    def_info = self.form_panel.attribute_widgets.get(
                        def_id, {})
                    def_dict = def_info.get("def", {})
                    name = def_dict.get(
                        "display_name") or def_dict.get("key") or def_id
                    attr_errors.append(
                        f"Atributo '{name}' deve ser um número inteiro válido.")

            if attr_errors:
                messagebox.showerror("Erros de Validação",
                                     "\n".join(attr_errors))
                return

        pos_values = {}
        if entity_name == "Player" and op in ("Create", "Update"):
            pos_values = self.form_panel.get_position_values()
            if not pos_values:
                messagebox.showerror(
                    "Erros de Validação", "O jogador deve ter pelo menos uma posição ativada.")
                return

        tp_enabled, tp_values, pf_values = False, {}, []
        if entity_name == "Manager" and op in ("Create", "Update"):
            tp_enabled, tp_values, pf_values = self.form_panel.get_tactical_profile_values()
            if tp_enabled and len(pf_values) > 3:
                messagebox.showerror("Erros de Validação", "Um técnico pode ter no máximo 3 formações preferidas no Perfil Tático.")
                return

        if isinstance(spec, CompositeEntitySpec):
            if op == "Delete":
                target_id = self.op_selector.get_selected_target_id()
                if not target_id:
                    messagebox.showerror(
                        "Erro", "Selecione o registro alvo para deletar.")
                    return
                stmts = []
                if entity_name == "Manager":
                    stmts.extend(build_manager_tactical_profile_delete_statements(target_id))
                    stmts.extend(
                        build_attribute_delete_statements(
                            "manager_attributes", "manager_id", target_id)
                    )
                stmts.extend(
                    build_composite_delete_statements(spec, target_id))
                self.sql_output.set_sql("\n".join(stmts))
                return

            values = self.form_panel.get_values()
            errors = []
            for field in spec.get_all_fields():
                val = values.get(field.name)
                field_errs = validate_field_spec(field, val)
                errors.extend(field_errs)

            if errors:
                messagebox.showerror("Erros de Validação", "\n".join(errors))
                return

            if spec.cross_validator:
                cross_errs = spec.cross_validator(values)
                if cross_errs:
                    messagebox.showerror(
                        "Erros de Validação Cruzada", "\n".join(cross_errs))
                    return

            if op == "Create":
                pk_field = spec.primary_key_field()
                shared_id = values.get(pk_field.name) if pk_field else None
                stmts = build_composite_insert_statements(
                    spec, values, shared_id=shared_id)
                if entity_name == "Manager" and shared_id:
                    stmts.extend(
                        build_attribute_insert_statements(
                            "manager_attributes",
                            "manager_id",
                            str(shared_id),
                            attr_values,
                        )
                    )
                    if tp_enabled:
                        stmts.extend(build_manager_tactical_profile_insert_statements(str(shared_id), tp_values, pf_values))
                self.sql_output.set_sql("\n".join(stmts))

            elif op == "Update":
                pk_field = spec.primary_key_field()
                if not pk_field:
                    messagebox.showerror(
                        "Erro", "A entidade não possui chave primária definida.")
                    return

                target_id = self.op_selector.get_selected_target_id() or values.get(pk_field.name)
                if not target_id:
                    messagebox.showerror(
                        "Erro", "Chave primária do registro alvo não identificada.")
                    return

                original = self.app_state.original_record_data
                changed_values = {}
                for field in spec.get_all_fields():
                    if field.primary_key:
                        continue
                    current_val = values.get(field.name)
                    orig_val = original.get(
                        field.column, original.get(field.name))

                    if (
                        str(current_val) != str(orig_val)
                        if (current_val is not None and orig_val is not None)
                        else current_val != orig_val
                    ):
                        changed_values[field.name] = current_val

                stmts = build_composite_update_statements(
                    spec, target_id, changed_values if original else values)
                if entity_name == "Manager":
                    stmts.extend(
                        build_attribute_update_statements(
                            "manager_attributes",
                            "manager_id",
                            target_id,
                            attr_values,
                            self.app_state.original_attribute_data,
                        )
                    )
                    stmts.extend(build_manager_tactical_profile_update_statements(
                        target_id,
                        tp_enabled,
                        tp_values,
                        pf_values,
                        self.app_state.original_tactical_profile_data,
                        self.app_state.original_preferred_formations
                    ))

                if not stmts:
                    messagebox.showinfo(
                        "Aviso", "Nenhum campo ou atributo foi alterado para atualização.")
                    return

                self.sql_output.set_sql("\n".join(stmts))
            return

        if op == "Delete":
            target_id = self.op_selector.get_selected_target_id()
            if not target_id:
                messagebox.showerror(
                    "Erro", "Selecione o registro alvo para deletar.")
                return
            stmts = []
            if entity_name == "Player":
                stmts.extend(build_attribute_delete_statements(
                    "player_attributes", "player_id", target_id))
                stmts.extend(
                    build_player_position_delete_statements(target_id))
            stmts.append(build_delete_statement(spec, target_id))
            self.sql_output.set_sql("\n".join(stmts))
            return

        values = self.form_panel.get_values()
        errors = []

        for field in spec.fields:
            val = values.get(field.name)
            field_errs = validate_field_spec(field, val)
            errors.extend(field_errs)

        if errors:
            messagebox.showerror("Erros de Validação", "\n".join(errors))
            return

        if spec.cross_validator:
            cross_errs = spec.cross_validator(values)
            if cross_errs:
                messagebox.showerror(
                    "Erros de Validação Cruzada", "\n".join(cross_errs))
                return

        if op == "Create":
            stmts = [build_insert_statement(spec, values)]
            if entity_name == "Player" and values.get("id"):
                player_id = str(values["id"])
                stmts.extend(
                    build_player_position_insert_statements(
                        player_id,
                        pos_values,
                    )
                )
                stmts.extend(
                    build_attribute_insert_statements(
                        "player_attributes",
                        "player_id",
                        player_id,
                        attr_values,
                    )
                )
            self.sql_output.set_sql("\n".join(stmts))

        elif op == "Update":
            pk_field = spec.primary_key_field()
            if not pk_field:
                messagebox.showerror(
                    "Erro", "A entidade não possui chave primária definida.")
                return

            target_id = self.op_selector.get_selected_target_id() or values.get(pk_field.name)
            if not target_id:
                messagebox.showerror(
                    "Erro", "Chave primária do registro alvo não identificada.")
                return

            original = self.app_state.original_record_data
            changed_values = {}
            for field in spec.fields:
                if field.primary_key:
                    continue
                current_val = values.get(field.name)
                orig_val = original.get(field.column, original.get(field.name))

                if (
                    str(current_val) != str(orig_val)
                    if (current_val is not None and orig_val is not None)
                    else current_val != orig_val
                ):
                    changed_values[field.name] = current_val

            stmts = []
            update_main = build_update_statement(
                spec, target_id, changed_values if original else values)
            if update_main:
                stmts.append(update_main)

            if entity_name == "Player":
                stmts.extend(
                    build_player_position_update_statements(
                        target_id,
                        pos_values,
                        self.app_state.original_position_data,
                    )
                )
                stmts.extend(
                    build_attribute_update_statements(
                        "player_attributes",
                        "player_id",
                        target_id,
                        attr_values,
                        self.app_state.original_attribute_data,
                    )
                )

            if not stmts:
                messagebox.showinfo(
                    "Aviso", "Nenhum campo, posição ou atributo foi alterado para atualização.")
                return

            self.sql_output.set_sql("\n".join(stmts))
