
#[derive(Debug)]
pub struct Pokemon {
    nombre: String,
    tipo: String,
    nivel: u8,
}

type Link = Option<Box<Nodo>>;
struct Nodo {
    pokemon: Pokemon,
    siguiente: Link,
}

pub struct ListaEnlazada {
    cabeza: Link,
}

impl ListaEnlazada {
    pub fn nueva() -> Self {
        ListaEnlazada { cabeza: None }
    }

    pub fn insertar_al_frente(&mut self, pokemon: Pokemon) {
        let nuevo_nodo = Nodo {
            pokemon: pokemon,
            siguiente: self.cabeza.take(),
        };
        self.cabeza = Some(Box::new(nuevo_nodo));
    }

    pub fn imprimir_lista(&self) {
        let mut actual = &self.cabeza;
        while let Some(nodo) = actual {
            println!(
                "{} - Tipo: {}, Nivel: {}",
                nodo.pokemon.nombre, nodo.pokemon.tipo, nodo.pokemon.nivel
            );
            actual = &nodo.siguiente;
        }
    }

    pub fn pop(&mut self) -> Option<Pokemon> {
        self.cabeza.take().map(|nodo| {
            self.cabeza = nodo.siguiente;
            nodo.pokemon
        })
    }

    pub fn iter(&self) -> Iter {
        Iter { actual: self.cabeza.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut { actual: self.cabeza.as_deref_mut() }
    }

    pub fn insertar_al_final(&mut self, pokemon: Pokemon) {
        let mut enlace = &mut self.cabeza;
        while enlace.is_some() {
            enlace = &mut enlace.as_mut().unwrap().siguiente;
        }
        *enlace = Some(Box::new(Nodo {
            pokemon,
            siguiente: None,
        }));
    }

    pub fn buscar_pokemon(&self, nombre: &str) -> Option<&Pokemon> {
        let mut actual = &self.cabeza;
        while let Some(nodo) = actual {
            if nodo.pokemon.nombre == nombre {
                return Some(&nodo.pokemon);
            }
            actual = &nodo.siguiente;
        }
        None
    }
}

pub struct Iter<'a> {
    actual: Option<&'a Nodo>,
}

pub struct IterMut<'a> {
    actual: Option<&'a mut Nodo>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Pokemon;

    fn next(&mut self) -> Option<Self::Item> {
        self.actual.take().map(|nodo| {
            self.actual = nodo.siguiente.as_deref_mut();
            &mut nodo.pokemon
        })
    }
}


impl<'a> Iterator for Iter<'a> {
    type Item = &'a Pokemon;

    fn next(&mut self) -> Option<Self::Item> {
        self.actual.map(|nodo| {
            self.actual = nodo.siguiente.as_deref();
            &nodo.pokemon
        })
    }
}

fn main() {
    let mut lista = ListaEnlazada::nueva();

    let pok1 = Pokemon {
        nombre: String::from("Charmander"),
        tipo: String::from("Fuego"),
        nivel: 1,
    };

    let pok2 = Pokemon {
        nombre: String::from("Bulbasaur"),
        tipo: String::from("Planta"),
        nivel: 1,
    };

    let pok3 = Pokemon {
        nombre: String::from("Squirtle"),
        tipo: String::from("Agua"),
        nivel: 1,
    };

    lista.insertar_al_frente(pok1);
    lista.insertar_al_frente(pok2);
    lista.insertar_al_frente(pok3);

    for pokemon in lista.iter_mut() {
        pokemon.nivel += 1;
    }

    println!("Lista después de subir de nivel a los Pokémon:");
    lista.imprimir_lista();

    let pikachu = Pokemon {
        nombre: String::from("Pikachu"),
        tipo: String::from("Eléctrico"),
        nivel: 10,
    };

    lista.insertar_al_final(pikachu);

    println!("Lista después de insertar a Pikachu al final:");
    lista.imprimir_lista();

    if let Some(pokemon) = lista.buscar_pokemon("Pikachu") {
        println!(
            "Encontrado: {} - Tipo: {}, Nivel: {}",
            pokemon.nombre, pokemon.tipo, pokemon.nivel
        );
    } else {
        println!("Pokémon no encontrado.");
    }
}
