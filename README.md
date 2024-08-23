# Agenda en Rust

Este proyecto en Rust implementa una agenda que puede almacenar y gestionar contactos y eventos. Incluye funcionalidades para insertar, imprimir y filtrar eventos y contactos.

## Estructuras de Datos

El código define las siguientes estructuras y enumeraciones para representar contactos y eventos:

- **Contactos**
  - `Familiar`: Representa un contacto familiar con un nombre, número de teléfono y edad.
  - `Empresarial`: Representa un contacto empresarial con un nombre, número de teléfono, edad y empresa.

- **Eventos**
  - `Reuniones`: Representa una reunión con una fecha y un lugar.
  - `Sociales`: Representa un evento social con una fecha, lugar y motivo.

- **Enumeraciones**
  - `TipoDeEvento`: Enum que puede ser una `Reuniones` o `Sociales`.
  - `TipoDeContacto`: Enum que puede ser un `Familiar` o `Empresarial`.
  - `TipoDeObjeto`: Enum que puede ser un `Contacto` o `Evento`.

## Funcionalidades

1. **Inserción de Datos**
   - `insertar_evento`: Inserta un evento en la agenda.
   - `insertar_contacto`: Inserta un contacto en la agenda.

2. **Impresión de la Agenda**
   - `imprimir_agenda`: Imprime todos los contactos y eventos en la agenda.

3. **Filtrado de Datos**
   - `filter2`: Filtra los elementos de la agenda basándose en un criterio proporcionado.
   - `es_del_2024`: Criterio de filtrado que selecciona eventos del año 2024.

## Uso

1. **Definir Contactos y Eventos**

   Puedes definir contactos y eventos con las siguientes estructuras:

   ```rust
   let familiar = Familiar {
       name: "Juan Pérez".to_string(),
       numero_de_telefono: "123456789".to_string(),
       age: 35,
   };

   let empresarial = Empresarial {
       name: "Empresa XYZ".to_string(),
       numero_de_telefono: "987654321".to_string(),
       age: 45,
       company: "XYZ Corp".to_string(),
   };

   let reunion = Reuniones {
       fecha: "2024-08-23".to_string(),
       lugar: "Oficina Central".to_string(),
   };

   let evento_social = Sociales {
       fecha: "2024-08-24".to_string(),
       lugar: "Parque Central".to_string(),
       motivo: "Fiesta de Aniversario".to_string(),
   };



## Funcionalidades

1. **Inserción de Datos**
   - `insertar_evento`: Inserta un evento en la agenda.
   - `insertar_contacto`: Inserta un contacto en la agenda.

2. **Impresión de la Agenda**
   - `imprimir_agenda`: Imprime todos los contactos y eventos en la agenda.

3. **Filtrado de Datos**
   - `filter2`: Filtra los elementos de la agenda basándose en un criterio proporcionado.
   - `es_del_2024`: Criterio de filtrado que selecciona eventos del año 2024.

## Uso

### Insertar Datos en la Agenda

Utiliza las funciones `insertar_evento` e `insertar_contacto` para agregar elementos a la agenda:

```rust
let mut agenda: Vec<TipoDeObjeto> = Vec::new();

insertar_evento(TipoDeEvento::Reuniones(reunion), &mut agenda);
insertar_evento(TipoDeEvento::Sociales(evento_social), &mut agenda);
insertar_contacto(TipoDeContacto::Familiar(familiar), &mut agenda);
insertar_contacto(TipoDeContacto::Empresarial(empresarial), &mut agenda);

```


## Imprimir y Filtrar la Agenda

Para imprimir la agenda completa y filtrar eventos del año 2024, utiliza las funciones correspondientes:

```rust
println!("Agenda completa:");
imprimir_agenda(&agenda);

let eventos_2024 = filter2(&agenda, es_del_2024);

println!("\nEventos del año 2024:");
imprimir_agenda(&eventos_2024);
```

# Ejecución
Para ejecutar el código, compílalo y corre el programa con cargo run en la raíz del proyecto. Asegúrate de tener Rust instalado en tu sistema.

```
cargo run
```