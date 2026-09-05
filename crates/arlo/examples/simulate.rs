fn main() {
    let rt = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    rt.block_on(async {
        println!("Executando bateria de testes e simulação com dados mockados...");
        let results = arlo_runtime::validation::run_full_validation_and_simulation();
        println!("------------------------------------------------------------");
        println!("Determinismo: {}", if results.determinism_passed { "APROVADO" } else { "REPROVADO" });
        println!("Sensibilidade Monotônica: {}", if results.monotonic_passed { "APROVADO" } else { "REPROVADO" });
        println!("Ausência de Monopólio: {}", if results.monopoly_passed { "APROVADO" } else { "REPROVADO" });
        println!("Regra do Mandante: {}", if results.home_possession_passed { "APROVADO" } else { "REPROVADO" });
        println!("Fronteira 0.7s: {}", if results.boundary_07_passed { "APROVADO" } else { "REPROVADO" });
        println!("Turnover sem Out: {}", if results.turnover_without_out_passed { "APROVADO" } else { "REPROVADO" });
        println!("------------------------------------------------------------");
        println!("Jogadas na partida simulada: {}", results.simulation_outcomes_count);
        println!("Relatório exportado para 'validation_report.md' e 'reports/validation_report.md'.");
    });
}