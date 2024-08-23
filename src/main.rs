#[derive(Clone)]
struct Familiar {
    name: String,
    numero_de_telefono: String,
    age: u8,
}

#[derive(Clone)]
struct Empresarial {
    name: String,
    numero_de_telefono: String,
    age: u8,
    company: String,
}

#[derive(Debug, Clone)]
struct Reuniones {
    fecha: String,
    lugar: String,
}

#[derive(Debug, Clone)]
struct Sociales {
    fecha: String,
    lugar: String,
    motivo: String,
}

#[derive(Clone)]
enum TipoDeEvento {
    Reuniones(Reuniones),
    Sociales(Sociales),
}

#[derive(Clone)]
enum TipoDeContacto {
    Familiar(Familiar),
    Empresarial(Empresarial),
}

#[derive(Clone)]
enum TipoDeObjeto {
    Contacto(TipoDeContacto),
    Evento(TipoDeEvento),
}

// Función para insertar eventos en la agenda
fn insertar_evento(evento: TipoDeEvento, agenda: &mut Vec<TipoDeObjeto>) {
    agenda.push(TipoDeObjeto::Evento(evento));
}

// Función para insertar contactos en la agenda
fn insertar_contacto(contacto: TipoDeContacto, agenda: &mut Vec<TipoDeObjeto>) {
    agenda.push(TipoDeObjeto::Contacto(contacto));
}

// Función para imprimir la agenda
fn imprimir_agenda(agenda: &Vec<TipoDeObjeto>) {
    for item in agenda {
        match item {
            TipoDeObjeto::Contacto(contacto) => match contacto {
                TipoDeContacto::Familiar(familiar) => {
                    println!(
                        "Contacto Familiar: {}, Tel: {}, Edad: {}",
                        familiar.name, familiar.numero_de_telefono, familiar.age
                    );
                }
                TipoDeContacto::Empresarial(empresarial) => {
                    println!(
                        "Contacto Empresarial: {}, Tel: {}, Edad: {}, Empresa: {}",
                        empresarial.name, empresarial.numero_de_telefono, empresarial.age, empresarial.company
                    );
                }
            },
            TipoDeObjeto::Evento(evento) => match evento {
                TipoDeEvento::Reuniones(reunion) => {
                    println!("Reunión en {} el día {}", reunion.lugar, reunion.fecha);
                }
                TipoDeEvento::Sociales(social) => {
                    println!(
                        "Evento Social en {} el día {} por motivo de {}",
                        social.lugar, social.fecha, social.motivo
                    );
                }
            },
        }
    }
}

// Implementación de la función filter2
fn filter2<T>(lista: &Vec<T>, criterio: impl Fn(&T) -> bool) -> Vec<T>
where
    T: Clone,
{
    let mut result = Vec::new();
    for item in lista {
        if criterio(item) {
            result.push(item.clone());
        }
    }
    result
}

fn es_del_2024(evento: &TipoDeObjeto) -> bool {
    match evento {
        TipoDeObjeto::Evento(TipoDeEvento::Reuniones(reunion)) => reunion.fecha.contains("2024"),
        TipoDeObjeto::Evento(TipoDeEvento::Sociales(social)) => social.fecha.contains("2024"),
        _ => false,
    }
}

fn main() {
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

    let reuniones = vec![
        Reuniones {
            fecha: "2024-09-23".to_string(),
            lugar: "Oficina del caribe".to_string(),
        },
        Reuniones {
            fecha: "2023-05-12".to_string(),
            lugar: "Sala de Juntas A".to_string(),
        },
        Reuniones {
            fecha: "2022-11-30".to_string(),
            lugar: "Hotel Plaza".to_string(),
        },
        Reuniones {
            fecha: "2021-02-15".to_string(),
            lugar: "Centro de Convenciones".to_string(),
        },
        Reuniones {
            fecha: "2024-12-01".to_string(),
            lugar: "Auditorio Principal".to_string(),
        },
    ];

    let eventos = vec![
        Sociales {
            fecha: "2024-10-24".to_string(),
            lugar: "Parque Central".to_string(),
            motivo: "Fiesta de divorcio".to_string(),
        },
        Sociales {
            fecha: "2023-06-10".to_string(),
            lugar: "Restaurante El Gourmet".to_string(),
            motivo: "Cena de Negocios".to_string(),
        },
        Sociales {
            fecha: "2024-07-22".to_string(),
            lugar: "Club Social".to_string(),
            motivo: "Fiesta de laboratorio".to_string(),
        },
        Sociales {
            fecha: "2021-03-18".to_string(),
            lugar: "Galería de Arte".to_string(),
            motivo: "Exposición de Arte".to_string(),
        },
    ];


    let mut agenda: Vec<TipoDeObjeto> = Vec::new();

    for reunion in reuniones {
        insertar_evento(TipoDeEvento::Reuniones(reunion), &mut agenda);
    }

    // Insertar eventos sociales en la agenda
    for evento_social in eventos {
        insertar_evento(TipoDeEvento::Sociales(evento_social), &mut agenda);
    }

    // Insertar contactos y eventos en la agenda
    insertar_contacto(TipoDeContacto::Familiar(familiar), &mut agenda);
    insertar_contacto(TipoDeContacto::Empresarial(empresarial), &mut agenda);
    insertar_evento(TipoDeEvento::Reuniones(reunion), &mut agenda);
    insertar_evento(TipoDeEvento::Sociales(evento_social), &mut agenda);

    println!("Agenda completa:");
    imprimir_agenda(&agenda);

    // Filtrar la agenda para obtener solo los eventos del año 2024
    let eventos_2024 = filter2(&agenda, es_del_2024);

    println!("\nEventos del año 2024:");
    imprimir_agenda(&eventos_2024);
}
