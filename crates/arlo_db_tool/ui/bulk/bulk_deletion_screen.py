import tkinter as tk
from tkinter import messagebox, ttk
from typing import Dict, List, Optional
from arlo_db_tool.app_state import AppState
from arlo_db_tool.db.connection import get_db_connection
from arlo_db_tool.db.reference_lookup import get_options
from arlo_db_tool.deletion.cascade_delete_builder import generate_cascade_delete_sql
from arlo_db_tool.deletion.deletion_preview_counter import (
    DeletionPreviewResult,
    build_deletion_preview,
)
from arlo_db_tool.deletion.filter_criteria import (
    resolve_all_competitions,
    resolve_all_managers,
    resolve_all_players,
    resolve_all_referees,
    resolve_all_teams,
    resolve_competition,
    resolve_country,
    resolve_federation,
    resolve_managers_of_team,
    resolve_players_of_nationality,
    resolve_players_of_team,
    resolve_referees_of_league,
    resolve_team,
    resolve_teams_of_league,
)
from arlo_db_tool.ui.sql_output_panel import SqlOutputPanel

TARGET_MODES = [
    "Jogadores de um Time",
    "Time Completo (com elenco e técnico)",
    "Times de uma Liga",
    "Jogadores por Nacionalidade",
    "Técnicos de um Time",
    "Juízes de uma Liga",
    "Competição / Liga Completa",
    "País Completo",
    "Federação Completa",
    "Todos os Jogadores",
    "Todos os Técnicos",
    "Todos os Juízes",
    "Todos os Times",
    "Todas as Competições",
]


class BulkDeletionScreen(ttk.Frame):
    def __init__(self, parent: tk.Widget, app_state: AppState):
        super().__init__(parent)
        self.app_state = app_state
        self.options_map: Dict[str, str] = {}
        self.resolved_ids: Dict[str, List[str]] = {}
        self.preview_result: Optional[DeletionPreviewResult] = None

        top_frame = ttk.LabelFrame(self, text="Critérios de Seleção para Exclusão", padding=(10, 8))
        top_frame.pack(fill=tk.X, padx=8, pady=6)

        row1 = ttk.Frame(top_frame)
        row1.pack(fill=tk.X, pady=(0, 6))

        lbl_mode = ttk.Label(row1, text="Alvo da Exclusão:")
        lbl_mode.pack(side=tk.LEFT, padx=(0, 6))

        self.mode_var = tk.StringVar(value=TARGET_MODES[0])
        self.combo_mode = ttk.Combobox(
            row1,
            textvariable=self.mode_var,
            values=TARGET_MODES,
            state="readonly",
            width=38,
        )
        self.combo_mode.pack(side=tk.LEFT, padx=(0, 16))
        self.combo_mode.bind("<<ComboboxSelected>>", self._on_mode_change)

        self.target_entity_frame = ttk.Frame(top_frame)
        self.target_entity_frame.pack(fill=tk.X, pady=(0, 6))

        self.lbl_entity = ttk.Label(self.target_entity_frame, text="Registro:")
        self.lbl_entity.pack(side=tk.LEFT, padx=(0, 6))

        self.entity_var = tk.StringVar()
        self.combo_entity = ttk.Combobox(
            self.target_entity_frame,
            textvariable=self.entity_var,
            state="readonly",
            width=50,
        )
        self.combo_entity.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=(0, 8))

        self.comp_options_frame = ttk.Frame(top_frame)
        self.comp_include_teams_var = tk.BooleanVar(value=False)
        self.chk_include_teams = ttk.Checkbutton(
            self.comp_options_frame,
            text="Incluir e excluir também todos os times cadastrados nesta competição",
            variable=self.comp_include_teams_var,
        )
        self.chk_include_teams.pack(side=tk.LEFT)

        action_row = ttk.Frame(top_frame)
        action_row.pack(fill=tk.X, pady=(4, 0))

        self.btn_preview = ttk.Button(
            action_row,
            text="Calcular Prévia de Exclusão",
            command=self._calculate_preview,
        )
        self.btn_preview.pack(side=tk.LEFT, padx=(0, 8))

        self.btn_generate_sql = ttk.Button(
            action_row,
            text="Gerar SQL de Exclusão",
            command=self._generate_sql,
            state="disabled",
        )
        self.btn_generate_sql.pack(side=tk.LEFT)

        middle_frame = ttk.LabelFrame(self, text="Prévia de Registros Afetados por Tabela", padding=(10, 8))
        middle_frame.pack(fill=tk.BOTH, expand=True, padx=8, pady=4)

        self.lbl_summary = ttk.Label(
            middle_frame,
            text="Nenhuma prévia calculada. Selecione os critérios acima e clique em 'Calcular Prévia'.",
            font=("TkDefaultFont", 9, "bold"),
        )
        self.lbl_summary.pack(anchor="w", pady=(0, 6))

        tree_frame = ttk.Frame(middle_frame)
        tree_frame.pack(fill=tk.BOTH, expand=True)

        self.tree = ttk.Treeview(
            tree_frame,
            columns=("table", "count"),
            show="headings",
            height=6,
        )
        self.tree.heading("table", text="Tabela")
        self.tree.heading("count", text="Registros a Deletar")
        self.tree.column("table", width=400, anchor="w")
        self.tree.column("count", width=150, anchor="e")

        tree_scroll = ttk.Scrollbar(tree_frame, orient=tk.VERTICAL, command=self.tree.yview)
        self.tree.configure(yscrollcommand=tree_scroll.set)
        self.tree.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        tree_scroll.pack(side=tk.RIGHT, fill=tk.Y)

        warning_frame = ttk.Frame(self, padding=(8, 4))
        warning_frame.pack(fill=tk.X, padx=8)

        lbl_warning = ttk.Label(
            warning_frame,
            text="Aviso de Segurança: Ao executar no DB Browser / SQLite CLI, confirme que 'Foreign Keys' está ativo no banco para garantir integridade referencial.",
            foreground="#b85d00",
            wraplength=900,
        )
        lbl_warning.pack(anchor="w")

        self.sql_output = SqlOutputPanel(self)
        self.sql_output.pack(fill=tk.X, padx=8, pady=(0, 6))

        self.reload_references()

    def _on_mode_change(self, _event):
        self.reload_references()
        self.btn_generate_sql.config(state="disabled")
        self.tree.delete(*self.tree.get_children())
        self.lbl_summary.config(
            text="Critério alterado. Clique em 'Calcular Prévia de Exclusão' para recalcular."
        )

    def reload_references(self):
        mode = self.mode_var.get()
        self.options_map.clear()

        if mode in ("Jogadores de um Time", "Time Completo (com elenco e técnico)", "Técnicos de um Time"):
            self.target_entity_frame.pack(fill=tk.X, pady=(0, 6))
            self.lbl_entity.config(text="Time:")
            self.comp_options_frame.pack_forget()
            opts = get_options(self.app_state.database_path, "teams", id_col="id", label_expr="name")
        elif mode in ("Times de uma Liga", "Juízes de uma Liga"):
            self.target_entity_frame.pack(fill=tk.X, pady=(0, 6))
            self.lbl_entity.config(text="Liga:")
            self.comp_options_frame.pack_forget()
            opts = get_options(
                self.app_state.database_path, "competitions", id_col="id", label_expr="name", where="kind = 'League'"
            )
        elif mode == "Competição / Liga Completa":
            self.target_entity_frame.pack(fill=tk.X, pady=(0, 6))
            self.lbl_entity.config(text="Competição:")
            self.comp_options_frame.pack(fill=tk.X, pady=(0, 6))
            opts = get_options(self.app_state.database_path, "competitions", id_col="id", label_expr="name")
        elif mode in ("Jogadores por Nacionalidade", "País Completo"):
            self.target_entity_frame.pack(fill=tk.X, pady=(0, 6))
            self.lbl_entity.config(text="País:")
            self.comp_options_frame.pack_forget()
            opts = get_options(self.app_state.database_path, "countries", id_col="id", label_expr="name")
        elif mode == "Federação Completa":
            self.target_entity_frame.pack(fill=tk.X, pady=(0, 6))
            self.lbl_entity.config(text="Federação:")
            self.comp_options_frame.pack_forget()
            opts = get_options(self.app_state.database_path, "federations", id_col="id", label_expr="name")
        else:
            self.target_entity_frame.pack_forget()
            self.comp_options_frame.pack_forget()
            opts = []

        display_list = []
        for r_id, label in opts:
            disp = f"{label} [{r_id}]"
            display_list.append(disp)
            self.options_map[disp] = str(r_id)

        self.combo_entity["values"] = display_list
        if display_list:
            self.combo_entity.current(0)
        else:
            self.entity_var.set("")

    def _calculate_preview(self):
        if not self.app_state.database_path:
            messagebox.showerror("Erro", "Nenhum banco de dados SQLite selecionado.")
            return

        mode = self.mode_var.get()
        selected_disp = self.entity_var.get()
        target_id = self.options_map.get(selected_disp)

        needs_target = mode not in (
            "Todos os Jogadores",
            "Todos os Técnicos",
            "Todos os Juízes",
            "Todos os Times",
            "Todas as Competições",
        )

        if needs_target and not target_id:
            messagebox.showerror("Erro", "Selecione um registro alvo para calcular a prévia.")
            return

        try:
            with get_db_connection(self.app_state.database_path) as conn:
                if mode == "Jogadores de um Time":
                    desc = f"Jogadores do Time '{selected_disp}'"
                    self.resolved_ids = resolve_players_of_team(conn, target_id)
                elif mode == "Time Completo (com elenco e técnico)":
                    desc = f"Time '{selected_disp}' e seus vínculos completos"
                    self.resolved_ids = resolve_team(conn, target_id)
                elif mode == "Times de uma Liga":
                    desc = f"Times vinculados à Liga '{selected_disp}'"
                    self.resolved_ids = resolve_teams_of_league(conn, target_id)
                elif mode == "Jogadores por Nacionalidade":
                    desc = f"Jogadores com nacionalidade do País '{selected_disp}'"
                    self.resolved_ids = resolve_players_of_nationality(conn, target_id)
                elif mode == "Técnicos de um Time":
                    desc = f"Técnico(s) do Time '{selected_disp}'"
                    self.resolved_ids = resolve_managers_of_team(conn, target_id)
                elif mode == "Juízes de uma Liga":
                    desc = f"Juízes da Liga '{selected_disp}'"
                    self.resolved_ids = resolve_referees_of_league(conn, target_id)
                elif mode == "Competição / Liga Completa":
                    include_teams = self.comp_include_teams_var.get()
                    desc = f"Competição '{selected_disp}' (Incluir times: {include_teams})"
                    self.resolved_ids = resolve_competition(
                        conn,
                        target_id,
                        include_teams=include_teams,
                        include_referees=True,
                    )
                elif mode == "País Completo":
                    desc = f"País '{selected_disp}' e todas as entidades vinculadas"
                    self.resolved_ids = resolve_country(conn, target_id)
                elif mode == "Federação Completa":
                    desc = f"Federação '{selected_disp}' e suas competições"
                    self.resolved_ids = resolve_federation(conn, target_id)
                elif mode == "Todos os Jogadores":
                    desc = "Todos os Jogadores do Banco de Dados"
                    self.resolved_ids = resolve_all_players(conn)
                elif mode == "Todos os Técnicos":
                    desc = "Todos os Técnicos do Banco de Dados"
                    self.resolved_ids = resolve_all_managers(conn)
                elif mode == "Todos os Juízes":
                    desc = "Todos os Juízes do Banco de Dados"
                    self.resolved_ids = resolve_all_referees(conn)
                elif mode == "Todos os Times":
                    desc = "Todos os Times e seus elencos do Banco de Dados"
                    self.resolved_ids = resolve_all_teams(conn)
                elif mode == "Todas as Competições":
                    desc = "Todas as Competições do Banco de Dados"
                    self.resolved_ids = resolve_all_competitions(conn, include_teams=False)
                else:
                    self.resolved_ids = {}
                    desc = "Alvo desconhecido"

                self.preview_result = build_deletion_preview(desc, self.resolved_ids)

        except Exception as e:
            messagebox.showerror("Erro ao Calcular Prévia", str(e))
            return

        self.tree.delete(*self.tree.get_children())
        for item in self.preview_result.summaries:
            self.tree.insert("", tk.END, values=(item.table_name, item.count))

        self.lbl_summary.config(
            text=f"Alvo: {self.preview_result.target_description}  •  Total de Registros a Deletar: {self.preview_result.total_records}"
        )

        if self.preview_result.has_deletions:
            self.btn_generate_sql.config(state="normal")
        else:
            self.btn_generate_sql.config(state="disabled")
            messagebox.showinfo("Prévia", "Nenhum registro encontrado para os critérios selecionados.")

    def _generate_sql(self):
        if not self.preview_result or not self.preview_result.has_deletions:
            messagebox.showerror("Erro", "Nenhuma prévia de exclusão calculada.")
            return

        sql_text = generate_cascade_delete_sql(self.resolved_ids)
        self.sql_output.set_sql(sql_text)