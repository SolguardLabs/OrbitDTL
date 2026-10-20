# Operación y conciliación

## Secuencia local

```bash
bun install --frozen-lockfile
cargo run --locked -- demo --json > report.json
bun run inspect report.json
bash scripts/ci.sh
```

El inspector es de solo lectura. Agrupa vaults y liquidaciones por ID de activo,
valida que `gross_out = net_out + fee` en cada evento y presenta agregados en
cadenas decimales. No interpreta el reporte como confirmación externa.

## Conciliación por sesión

1. Guardar revisión, toolchain y comando exacto de generación.
2. Relacionar `intent_queued`, `intent_cancelled` e `intent_settled` por ID.
3. Confirmar que cada liquidación tiene beneficiario, activo destino, bruto,
   neto y comisión consistentes.
4. Conciliar pagos del vault destino y créditos al beneficiario y operador.
5. Revisar estados de sesión y límites de riesgo con el orden de eventos; los
   totales aislados no reflejan necesariamente el orden de autorización.

El repositorio no versiona `target/`, `node_modules/`, reportes generados ni
credenciales. Ejecutar CI antes de publicar cambios a rutas, pricing, riesgo
o contabilidad. Un cambio documental no sustituye la verificación de la
revisión exacta etiquetada.
