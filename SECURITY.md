# Seguridad

Orbit DTL esta disenado como un entorno de laboratorio para revisar sistemas de
liquidacion diferida, gestion de vaults y control de riesgo en flujos
multi-activo. El proyecto expone una arquitectura de protocolo realista con
validaciones de activos, rutas, sesiones, oraculos, limites operativos y eventos
auditables.

## Modelo De Seguridad

El protocolo se apoya en varias capas de control:

- registro explicito de activos habilitados;
- vaults asociados a un unico activo y controlador;
- rutas configuradas con vault origen, vault destino, limites y parametros de
  comision;
- precios con umbral minimo de confianza;
- intents con beneficiario, importe minimo de salida, deadline y nonce;
- sesiones de liquidacion con registro de counterflow;
- motor de riesgo con limites por cuenta, ruta y sesion;
- eventos serializables para auditoria posterior.

## Invariantes Esperadas

Durante la operacion normal se espera que:

- un vault no liquide activos distintos al activo que tiene asignado;
- las rutas deshabilitadas no puedan ejecutar liquidaciones;
- una orden pendiente solo pueda liquidarse una vez;
- las sesiones registren las operaciones incluidas;
- los pagos respeten el importe minimo solicitado por el usuario;
- las comisiones se contabilicen hacia el operador de ruta;
- los limites de riesgo se evaluen antes de mover fondos;
- la salida JSON del binario sea determinista para pruebas de integracion.

## Practicas De Desarrollo

El repositorio incluye validaciones automatizadas para reducir regresiones:

- `cargo fmt --all -- --check`;
- `cargo build --all-targets --locked`;
- `cargo test --locked`;
- `cargo clippy --all-targets --all-features --locked -- -D warnings`;
- `bun run fmt:check`;
- `bun run build`;
- `bun test --timeout 30000 ./tests/node`.

Estas comprobaciones se ejecutan en GitHub Actions mediante `scripts/ci.sh`.

## Gestion De Dependencias

Las dependencias Rust quedan fijadas en `Cargo.lock`. Las dependencias de Bun
quedan fijadas en `bun.lock`. Dependabot revisa actualizaciones de Cargo,
npm/Bun y GitHub Actions con una cadencia semanal.

## Reporte De Incidencias

Este laboratorio no debe recibir reportes publicos de seguridad como si fuera
un protocolo desplegado en produccion. Para ejercicios internos, documenta el
hallazgo con:

- descripcion tecnica reproducible;
- impacto economico o de disponibilidad;
- archivos y funciones afectadas;
- precondiciones necesarias;
- pasos de reproduccion;
- propuesta de mitigacion;
- comandos usados para verificar el resultado.

## Alcance

El alcance de revision incluye el codigo Rust del binario, los scripts de CI,
los tests Rust, los tests Node y la configuracion de GitHub Actions. Quedan
fuera integraciones externas, despliegues, claves privadas, infraestructura de
nube y cualquier red de produccion.
