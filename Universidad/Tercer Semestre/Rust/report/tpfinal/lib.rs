#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::tpfinal::TpfinalRef;
#[ink::contract]

mod tpfinal {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use chrono::prelude::*;
    use chrono::{Duration,NaiveDateTime};
   
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Actividad {
        Futbol, 
        Basquet, 
        Rugby,
        Hockey, 
        Natacion, 
        Tenis, 
        Paddle,
    }

    impl Actividad{
        // Se envia un String, y se devuelve option del enum Actividad
        pub fn match_actividad(actividad: String) -> Option<Actividad> {
            let actividad2 = actividad.to_lowercase(); // Lo convertimos a minusculas
            
            match actividad2.as_str() {
                "futbol" => Some(Actividad::Futbol),
                "basquet" => Some(Actividad::Basquet),
                "rugby" => Some(Actividad::Rugby),
                "hockey" => Some(Actividad::Hockey),
                "natacion" => Some(Actividad::Natacion),
                "tenis" => Some(Actividad::Tenis),
                "paddle" => Some(Actividad::Paddle),
                _ => None, // Retornar None Si no coincide.
            }
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct Costos{
        cat_a: u32,
        cat_b: u32,
        cat_c: u32,
        cant_pagos_consecutivos: u32,
        cant_descuento: u32
    }

    impl Costos{
        fn get_valor_costos(&self,categoria:&Categorias) -> u32 {
            match categoria {
                Categorias::A => self.cat_a, //Deberia retornar 5000
                Categorias::B => self.cat_b, //Deberia retornar 3000
                Categorias::C => self.cat_c, //Deberia retornar 2000
            }
         }
        
        
        fn new(cat_a:u32,cat_b:u32,cat_c:u32,cant_pagos_consecutivos:u32,cant_descuento:u32) -> Self{
            Costos{cat_a,cat_b,cat_c,cant_pagos_consecutivos,cant_descuento}
        }
    }

    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Categorias {
        A, // 5000 tokens, permite asistir al gimnasio y todas las actividades
        B, // 3000 tokens, permite asistir al gimnasio y una actividad a elección
        C  // 2000 tokens, permite asistir solo al gimnasio
    }   

    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct Socio {
        nombre: String,
        dni: String,
        categoria: Categorias,
        actividad: Option<Actividad>,
        pagos_consecutivos: u32,
    }

    impl Socio {
        pub fn new(nombre: String, dni: String, categoria: Categorias, actividad: Option<Actividad>,pagos_consecutivos:u32) -> Socio {	//New del Socio
            Socio{nombre,dni,categoria,actividad,pagos_consecutivos}
        }
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Pago {
        socio: Socio,
        categoria: Categorias,
        vencimiento: Fecha, 
        fecha_pago: Option<Fecha>,
        monto: u32,
        estado: bool,
    }

    impl Pago{
        fn new(socio:Socio,categoria:Categorias,vencimiento:Fecha,fecha_pago:Option<Fecha>,monto:u32,estado:bool)-> Pago {
            Pago{socio,categoria,vencimiento,fecha_pago,monto,estado}
        }
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Fecha {
        dia:u32,
        mes:u32,
        anio:u32
    }

    impl Fecha{
        fn new(dia: u32, mes: u32, anio: u32) -> Fecha {
            Fecha{ dia, mes, anio }
        }

        pub fn procesar_tiempo(&mut self, tiempo: u64) -> Fecha {
            let tiempo = tiempo as i64;
            let mut resultado =  Fecha::new(0, 0, 0);

            let datetime = NaiveDateTime::from_timestamp_opt(tiempo, 0);
            if let Some(datetime) = datetime {
                let date = datetime.date();
                resultado.dia = date.day();
                resultado.mes = date.month();
                resultado.anio = date.year() as u32;
            }

            return resultado;
        }

        fn crear_vencimiento(&mut self, dias_agregar: i64, tiempo: u64) {
            let tiempo = tiempo as i64;

            let fecha = NaiveDateTime::from_timestamp_opt(tiempo, 0);
            if let Some(fecha) = fecha {
                let fecha_mas_10_dias = fecha.checked_add_signed(Duration::days(dias_agregar)).unwrap();
                self.dia = fecha_mas_10_dias.day() as u32;
                self.mes = fecha_mas_10_dias.month();
                self.anio = fecha_mas_10_dias.year() as u32;
            }
        }
        
        pub fn es_mayor(&self, otra_fecha: &Fecha) -> bool {
            if self.anio > otra_fecha.anio {
                true
            } else if self.anio == otra_fecha.anio {
                if self.mes > otra_fecha.mes {
                    true
                } else if self.mes == otra_fecha.mes {
                    self.dia > otra_fecha.dia
                } else {
                    false
                }
            } else {
                false
            }
        }

        pub fn es_bisiesto(&self) -> bool{
            self.anio % 4 == 0
        }

        fn es_mes_valido(&self)-> bool{
            self.mes >= 1 && self.mes <= 12
        }

        fn es_dia_valido(&self) -> Result<()>{
            if self.dias_en_mes().is_ok(){
              if self.dia > 0 && self.dia <= self.dias_en_mes().unwrap() { return Ok(()) }
            }
            return Err(Error::FechaError);
        }

        fn dias_en_mes(&self) -> Result<u32> {
            match self.mes {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
                4 | 6 | 9 | 11 => Ok(30),
                2 => {
                    if self.es_bisiesto() {Ok(29)} 
                    else {Ok(28)}
                },
                _ => Err(Error::FechaError),
            }
        }

        pub fn es_fecha_valida(&self) -> Result<()>{
            if self.es_mes_valido() && self.es_dia_valido().is_ok() {
                return Ok(());
            } else {
                return Err(Error::FechaError);
            }
        }
    }

    #[ink(storage)]
    pub struct Tpfinal {
        costos:Costos,
        pagos: Vec<Pago>, // Usar el Vec del ink.
        socios: Vec<Socio>,
        owner: AccountId,
        direcciones_autorizadas: Vec<AccountId>,
        politica_de_autorizacion: bool,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Retornado cuando se busca un socio que no existe.
        SocioNotFoundError,
        /// Retornado cuando se intenta crear un socio ya existente.
        SocioExistenteError,
        /// Retornado cuando el monto ingresado es menor al necesario.
        MontoInsuficienteError,
        /// Retornado cuando el Socio elige una actividad teniendo cateogira A o C.
        SocioEligioActividadError,
        /// Retornado cuando la dirección no esta autorizada, por lo tanto la politica de autorización esta activada (true).
        DireccionNoAutorizadaError,
        /// Retornado cuando la dirección ya estaba autorizada.
        DireccionYaAutorizadaError,
        /// Retornado cuando la dirección se quiso eliminar y no estaba en el Vec.
        DireccionNoEliminadaError,
        /// Retornado cuando se quiere agregar una dirección, o modificar la política y no es el owner.
        NoEsElOwnerError,
        /// Retornado cuando se intenta crear una fecha que no exxiste.
        FechaError,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Tpfinal {
        // Implementacion de las direcciones autorizadas:
        fn es_direccion_valida(&self) -> bool {
            if !self.politica_de_autorizacion { return true } // Si no esta activada la política de autorizacion retorna true inmediatamente.
            let caller = self.env().caller();

            // Si el caller es el owner
            if caller == self.owner { return true }

            // Si el caller está en el Vec de las direcciones autorizadas
            if self.direcciones_autorizadas.contains(&caller) { return true }

            return false;
        }

        fn es_el_owner(&self) -> bool{
            self.owner == self.env().caller()
        }

        /// Recibe la dirección de una cuenta.
        /// Cambia el owner del contrato a la cuenta recibida.
        /// Necesita que el caller sea el owner del contrato.
        #[ink(message)]
        pub fn transferir_owner(&mut self, direccion: AccountId) -> Result<()> {
            self.priv_transferir_owner(direccion)
        }

        fn priv_transferir_owner(&mut self, direccion: AccountId) -> Result<()> {
            // Si no es el owner no puede transferir.
            if !self.es_el_owner() { return Err(Error::NoEsElOwnerError) }

            self.owner = direccion;
            Ok(())
        }

        /// Retorna el id del owner del contrato.
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Recibe la dirección de una cuenta.
        /// Pone la dirección recibida en el vector de direcciones autorizadas y si ya estaba retorna un error.
        /// Necesita que el caller sea el owner del contrato.
        #[ink(message)]
        pub fn agregar_direccion_autorizada(&mut self, direccion: AccountId) -> Result<()> {
            self.priv_agregar_direccion_autorizada(direccion)
        }

        fn priv_agregar_direccion_autorizada(&mut self, direccion: AccountId) -> Result<()> {
            // Si no es el owner no puede modificar las direcciones.
            if !self.es_el_owner() { return Err(Error::NoEsElOwnerError) }
 
            // Si ya esta en el Vec, no lo vuelve a agregar y retorna Error
            if self.direcciones_autorizadas.contains(&direccion) { return Err(Error::DireccionYaAutorizadaError) }
            self.direcciones_autorizadas.push(direccion);
            Ok(())
        }

        /// Recibe la dirección de una cuenta.
        /// Si encuentra la dirección recibida como autorizada la elimina del vector.
        /// Necesita que el caller sea el owner del contrato.
        #[ink(message)]
        pub fn eliminar_direccion_autorizada(&mut self, direccion: AccountId) -> Result<()> {
            self.priv_eliminar_direccion_autorizada(direccion)
        }

        fn priv_eliminar_direccion_autorizada(&mut self, direccion: AccountId) -> Result<()> {
            // Si no es el owner no puede modificar las direcciones.
            if !self.es_el_owner() { return Err(Error::NoEsElOwnerError) }

            // Si ya esta en el Vec, no lo vuelve a agregar y retorna Error
            if self.direcciones_autorizadas.contains(&direccion) {
                self.direcciones_autorizadas.retain(|&d| d != direccion);
                return Ok(());
            }
            // Si no esta en el Vec retorno Error.
            return Err(Error::DireccionNoEliminadaError);
        }

        /// Retorna la política de autorización.
        #[ink(message)]
        pub fn get_politica_de_autorizacion(&self) -> bool {
            self.politica_de_autorizacion
        }

        /// Cambia el valor de la política de autorización.
        /// Necesita que el caller sea el owner del contrato.
        #[ink(message)]
        pub fn intercambiar_politica_de_autorizacion(&mut self) -> Result<()> {
            self.priv_intercambiar_politica_de_autorizacion()
        }

        fn priv_intercambiar_politica_de_autorizacion(&mut self) -> Result<()> {
            // Si no es el owner no puede modificar la politica.
            if !self.es_el_owner() { return Err(Error::NoEsElOwnerError) }

            self.politica_de_autorizacion = !self.politica_de_autorizacion;
            Ok(())
        }

        /// Recibe los datos de un socio.
        /// Crea y agrega el socio al vector y genera un pago con sus datos.
        /// Necesita que el caller este autorizado.
        #[ink(message)]
        pub fn crear_socio(&mut self, nombre: String, dni: String, categoria: Categorias, actividad: Option<Actividad>) -> Result<()> {
            self.priv_crear_socio(nombre, dni, categoria, actividad)
        }

        fn priv_crear_socio(&mut self, nombre: String, dni: String, categoria: Categorias, actividad: Option<Actividad>) -> Result<()> {
            // Verificar que la direccion sea valida
            if !self.es_direccion_valida() { return Err(Error::DireccionNoAutorizadaError) }

            if self.retornar_socio(&dni).is_none() { // Verificar si no esta en el Vec
                // Retorna Error si se creo el socio con Categoria A, o C, y con una Actividad
                if (categoria == Categorias::A || categoria == Categorias::C) && actividad.is_some() { return Err(Error::SocioEligioActividadError) }

                let socio = Socio::new(nombre, dni, categoria.clone(), actividad,0);
                self.socios.push(socio.clone());

                let tiempo_actual = Self::env().block_timestamp() / 1000; 
                let mut vencimiento: Fecha = Fecha::new(0,0,0);
                vencimiento.crear_vencimiento(10, tiempo_actual);

                let pago = Pago::new(socio, categoria.clone(), vencimiento, None, self.costos.get_valor_costos(&categoria), false);           //MODIFICAR FECHA	//Creo un pago con 10 dias de vencimiento (hay que implementarlo)
                self.pagos.push(pago);  
                return Ok(());
            }
            
            Err(Error::SocioExistenteError)
        }

        /// Recibe el dni de un socio.
        /// Retorna al socio buscado o None si este no está.
        fn retornar_socio(& self, dni: &String) -> Option<&Socio> {
            self.socios.iter().find(|s|&s.dni == dni)
        }

        /// Recibe el dni de un socio.
        /// Retorna un pago suyo con estado false o None si no se encuentra.
        fn retornar_pago(&mut self, dni: &String) -> Option<&mut Pago> {
            self.pagos.iter_mut().find(|p| &p.socio.dni == dni && p.estado == false)
        }

        /// Recibe el dni de un socio.
        /// Si encuentra un socio con el dni recibido aumenta en 1 sus pagos consecutivos.
        fn aumentar_pagos_consecutivos(&mut self, dni: &String) {
            self.socios.iter_mut().find(|s| &s.dni == dni).unwrap().pagos_consecutivos += 1;
        }

        /// Recibe el nuevo valor de las categorías a, b, c, en ese orden.
        /// Cambia los costos de las categorías con los valores recibidos.
        /// Necesita que el caller este autorizado.
        #[ink(message)]
        pub fn set_valor_costos(&mut self, valor_cat_a: u32, valor_cat_b: u32, valor_cat_c: u32) -> Result<()> {
            self.priv_set_valor_costos(valor_cat_a, valor_cat_b, valor_cat_c)
        }
        
        fn priv_set_valor_costos(&mut self, valor_cat_a: u32, valor_cat_b: u32, valor_cat_c: u32) -> Result<()> {
            // Verificar que la direccion sea valida
            if !self.es_direccion_valida() { return Err(Error::DireccionNoAutorizadaError) }

            self.costos.cat_a = valor_cat_a;
            self.costos.cat_b = valor_cat_b;
            self.costos.cat_c = valor_cat_c;
            Ok(())
        }

        /// Se retorna el nombre y el dni (en un Vec de tupla), de los Socios con pagos vencidos.
        #[ink(message)]
        pub fn socios_pendientes(&self) -> Vec<(String,String)>{
            let tiempo_actual = Self::env().block_timestamp() / 1000; 
            let mut aux: Fecha = Fecha::new(0,0,0);
            aux = aux.procesar_tiempo(tiempo_actual); //procesamos el tiempo actual
            self.pagos.iter().filter(|&p| p.estado == false && !p.vencimiento.es_mayor(&aux)).map(|p| (p.socio.nombre.clone(), p.socio.dni.clone())).collect::<Vec<(String, String)>>()
        }
        
        /// Se retorna el nombre y el dni (en un Vec de tupla), de los Socios morosos según la actividad enviada por parámetro
        #[ink(message)]
        pub fn socios_no_morosos_por_actividad(&self, actividad: String) -> Option<Vec<(String, String)>> {
          let tiempo_actual = Self::env().block_timestamp() / 1000;
          let mut aux: Fecha = Fecha::new(0, 0, 0);
          aux = aux.procesar_tiempo(tiempo_actual); // procesamos el tiempo actual

          let actividad2 = Actividad::match_actividad(actividad);

          if actividad2.is_none() { return None }
          return Some(self.socios.iter()
            .filter(|&socio| self.no_es_socio_moroso(actividad2.clone().unwrap(), socio.clone(), aux.clone()))
            .map(|socio| (socio.nombre.clone(), socio.dni.clone()))
            .collect::<Vec<(String, String)>>());
        }

        fn no_es_socio_moroso(&self, actividad: Actividad, socio: Socio, fecha: Fecha) -> bool {
            // El gimnasio no cuenta como actividad, ya que todos pueden ir al gimnasio.
            // Tampoco contamos Categoria A, solo los que elijen una actividad en especifico y que tienen Categoria B.
            if socio.categoria == Categorias::B {
              if socio.actividad.unwrap() == actividad{
                let pagos = self.consultar_pagos(socio.dni).unwrap();
                if !fecha.es_mayor(&pagos[pagos.len() -1].vencimiento){return true}
              }
            }
            return false;
        }

        /// Recibe el dni de un socio.
        /// Si el socio existe crea un pago con sus datos.
        /// Necesita que la el caller este autorizado.
        #[ink(message)]
        pub fn crear_pago(&mut self, dni: String) -> Result<()> {
           self.priv_crear_pago(dni)
        }
        
        fn priv_crear_pago(&mut self, dni: String) -> Result<()> {
            // Verificar que la direccion sea valida
            if !self.es_direccion_valida() { return Err(Error::DireccionNoAutorizadaError) }

            let socio = match self.socios.iter_mut().find(|s| s.dni == dni) {
                Some(socio) => socio,
                None => return Err(Error::SocioNotFoundError)
            };

            let mut monto_pagar = self.costos.get_valor_costos(&socio.categoria);
            if socio.pagos_consecutivos == self.costos.cant_pagos_consecutivos{
                socio.pagos_consecutivos = 0;
                monto_pagar -= (monto_pagar * self.costos.cant_descuento) / 100;
            }

            let vector_pago: Vec<Pago> = self.pagos.iter().filter(|pago| pago.socio.dni == dni).cloned().collect(); // Tomamos el vector de pagos
            let mut tiempo_actual = Self::env().block_timestamp() / 1000; 
            let mut aux: Fecha = Fecha::new(0,0,0);
            let mut vencimiento: Fecha = Fecha::new(0,0,0);

            let pago_aux = vector_pago[vector_pago.len()-1].clone(); // Tomamos el ultimo pago, para comprobar si esta vencido
            aux.procesar_tiempo(tiempo_actual); // Procesamos el tiempo actual
            if pago_aux.vencimiento.es_mayor(&aux) {
                let fecha = NaiveDate::from_ymd_opt(
                    pago_aux.vencimiento.anio as i32,
                    pago_aux.vencimiento.mes,
                    pago_aux.vencimiento.dia,
                ).expect("Error al desempaquetar la fecha").and_hms_opt(0, 0, 0).expect("Error al desempaquetar la fecha"); // Pasamos el vencimiento del ultimo pago a Epoch unix
                tiempo_actual = fecha.timestamp() as u64;
            }
            vencimiento.crear_vencimiento(30, tiempo_actual);

            self.pagos.push(Pago::new(socio.clone(), socio.categoria.clone(), vencimiento, None, monto_pagar, false));
            Ok(())
        }

        /// Recibe el dni de un socio y el monto pagado.
        /// Si el socio existe y el monto corresponde con su categoría su pago pasa a estado true y se crea uno nuevo.
        /// Necesita que el caller este autorizado.
        #[ink(message)]
        pub fn pagar_pago(&mut self, dni: String, monto: u32) -> Result<()> {
            self.priv_pagar_pago(dni, monto)
        }

         fn priv_pagar_pago(&mut self, dni: String, monto: u32)-> Result<()> {
            // Verificar que la direccion sea valida
            if !self.es_direccion_valida() { return Err(Error::DireccionNoAutorizadaError) }

            if self.retornar_socio(&dni).is_some(){
                if let Some(pago) = self.retornar_pago(&dni){
                    if monto >= pago.monto {
                        pago.estado = true;

                        let tiempo_actual = Self::env().block_timestamp() / 1000; 
                        let mut tiempo_procesado: Fecha = Fecha::new(0,0,0);
                        tiempo_procesado = tiempo_procesado.procesar_tiempo(tiempo_actual);
                        pago.fecha_pago = Some(tiempo_procesado.clone());
                        
                        if pago.vencimiento.es_mayor(&tiempo_procesado) {
                            self.aumentar_pagos_consecutivos(&dni);
                        } else {
                            let socio = self.socios.iter_mut().find(|s| s.dni == dni).unwrap();
                            socio.pagos_consecutivos = 0;
                        }
                        return self.crear_pago(dni)
                    }
                    return Err(Error::MontoInsuficienteError);
                }
            }
            return Err(Error::SocioNotFoundError);
        }
        
        /// Recibe el dni de un socio.
        /// Retorna un vector con todos los pagos del socio o todos los pagos si no encuentra el dni.
        #[ink(message)]
        pub fn consultar_pagos(&self, dni: String) -> Option<Vec<Pago>> {	
            self.priv_consultar_pagos(dni) 
        }

        fn priv_consultar_pagos(&self, dni: String) -> Option<Vec<Pago>> {	
            let pagos_encontrados: Vec<Pago> = self.pagos.iter().filter(|pago| pago.socio.dni == dni).cloned().collect();
            if pagos_encontrados.is_empty() { Some(self.pagos.clone()) } else { Some(pagos_encontrados) }  // Si no encuentra dni, envia todos los pagos  
        }

        /// Recibe un dia, mes y año.
        /// Retorna la recaudación por categoría en el mes recibido.
        /// Si la fecha es inválida retorna todo en 0.
        #[ink(message)]
        pub fn recaudacion_mensual(&self, dia:u32, mes:u32 , anio:u32 ) -> (u32,u32,u32) {
            let aux = Fecha::new(dia,mes,anio);
            if aux.es_fecha_valida().is_ok() {
                return self.priv_recaudacion_mensual(aux);
            } else {
                // En caso de error devuelve toda la recaudacion de las categorias en 0
                return (0,0,0);
            }
        }

        pub fn priv_recaudacion_mensual(&self, fecha:Fecha) -> (u32,u32,u32){
        		// Filtramos los pagos del mismo Mes
            let pagos_mes = self.pagos.iter().filter(|p|p.fecha_pago.is_some() && p.fecha_pago.as_ref().unwrap().mes == fecha.mes).cloned().collect::<Vec<_>>();
            
            // Hacemos 3 Vec<Pago> segun su categoria
            let pagos_cat_a: Vec<_> = pagos_mes.iter().filter(|p|p.categoria == Categorias::A).collect();
            let pagos_cat_b: Vec<_> = pagos_mes.iter().filter(|p|p.categoria == Categorias::B).collect();
            let pagos_cat_c: Vec<_> = pagos_mes.iter().filter(|p|p.categoria == Categorias::C).collect();
            
            // Sumamos los montos
            let monto_cat_a = pagos_cat_a.iter().map(|pago| pago.monto).sum();
            let monto_cat_b = pagos_cat_b.iter().map(|pago| pago.monto).sum();
            let monto_cat_c = pagos_cat_c.iter().map(|pago| pago.monto).sum();
            
            // Retornamos los tres structs
            return (monto_cat_a,monto_cat_b,monto_cat_c);
        }

        /// Constructor.
        /// Recibe los montos de las categorías a, b y c, la cantidad
        /// de pagos consecutivos para un descuento y el valor del descuento.
        #[ink(constructor)]
        pub fn new(a: u32, b: u32, c: u32, cant_pagos_consecutivos: u32, cant_descuento: u32) -> Self {
            let owner = Self::env().caller();
            let costos = Costos::new(a, b, c, cant_pagos_consecutivos, cant_descuento);
            let pagos: Vec<Pago> = Vec::new();
            let socios: Vec<Socio> = Vec::new();
            let direcciones_autorizadas: Vec<AccountId> = Vec::new();
            Self {costos, pagos, socios, owner, direcciones_autorizadas, politica_de_autorizacion: true }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use super::Tpfinal;
        use super::Categorias;
        use super::Actividad;

        #[test]
        fn test_crear_usuario() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000,3000,2000,3,10);
            assert!(tpfinal.priv_crear_socio( "Rami".to_string(), "123456".to_string(), Categorias::A, None).is_ok());
            assert!(matches!(tpfinal.priv_crear_socio( "Rami".to_string(), "123456".to_string(), Categorias::B, Some(Actividad::Futbol)), Err(Error::SocioExistenteError)));  //Result Err de Socio Existente
            assert!(tpfinal.priv_crear_pago("123456".to_string()).is_ok());
            assert!(matches!(tpfinal.priv_crear_pago("Usuario inexistente".to_string()), Err(Error::SocioNotFoundError)));// No encuentra usuario, retorna false.
            tpfinal.priv_consultar_pagos("123456".to_string());
            tpfinal.priv_consultar_pagos("Usuario inexistente".to_string());  // Retorna todos los pagos si no encuentra el usuario
            assert!(matches!(tpfinal.priv_pagar_pago("123456".to_string(), 0), Err(Error::MontoInsuficienteError))); // Mando menos monto, retorna false
            assert!(matches!( tpfinal.priv_pagar_pago("Usuario inexistente".to_string(), 5000), Err(Error::SocioNotFoundError))); // No encuentra usuario, retorna otro false.
            assert!(tpfinal.priv_pagar_pago("123456".to_string(), 5000).is_ok());
            assert!(tpfinal.priv_set_valor_costos(6000,4000,3000).is_ok());
            let categoria = Categorias::A;
            assert_eq!(tpfinal.costos.get_valor_costos(&categoria),6000);
            assert_eq!(tpfinal.costos.get_valor_costos(&Categorias::B),4000);
            assert_eq!(tpfinal.costos.get_valor_costos(&Categorias::C),3000);
        }

        #[test]
        fn test_crear_socio() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Juan".to_string(), "12345".to_string(), Categorias::A,None).is_ok());
            
            let socio = tpfinal.retornar_socio(&"12345".to_string());
            assert!(socio.is_some());
            let socio = socio.unwrap();
            assert_eq!(socio.nombre, "Juan");
            assert_eq!(socio.categoria, Categorias::A);
            assert_eq!(socio.actividad, None);
            assert_eq!(socio.pagos_consecutivos, 0);
        }

        #[test]
        fn test_crear_socio_cat_a() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            let resultado = tpfinal.crear_socio("Juan".to_string(), "12345".to_string(), Categorias::A,None);
            assert!(resultado.is_ok());
            let socio = tpfinal.retornar_socio(&"12345".to_string());
            assert!(socio.is_some());
            let socio = socio.unwrap();
            assert_eq!(socio.nombre, "Juan");
            assert_eq!(socio.categoria, Categorias::A);
            assert_eq!(socio.actividad, None);
            assert_eq!(socio.pagos_consecutivos, 0);
            assert_eq!(tpfinal.socios.len(),1);
            assert_eq!(tpfinal.pagos.len(),1);
        }

        #[test]
        fn test_crear_socio_cat_a_error_actividad() { // Creo un socio con Categoria A, y con Actividad Futbol, retorna Err.
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            let result_crear_socio = tpfinal.crear_socio("Pelopincho".to_string(), "121212".to_string(), Categorias::A,Some(Actividad::Futbol));
            assert!(result_crear_socio.is_err());
            let socio = tpfinal.retornar_socio(&"121212".to_string());
            assert!(socio.is_none());
            assert_eq!(tpfinal.pagos.len(),0);  // No se tendria que haber pusheado nada a los Vec
            assert_eq!(tpfinal.socios.len(),0);
        }

        #[test]
        fn test_crear_socio_cat_c_error_actividad() { // Creo un socio con Categoria C, y con Actividad Basquet, retorna Err.
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            let result_crear_socio = tpfinal.crear_socio("Juan".to_string(), "12345".to_string(), Categorias::C,Some(Actividad::Basquet));
            assert!(result_crear_socio.is_err());
            let socio = tpfinal.retornar_socio(&"12345".to_string());
            assert!(socio.is_none());
            assert_eq!(tpfinal.pagos.len(),0);  // No se tendria que haber pusheado nada a los Vec
            assert_eq!(tpfinal.socios.len(),0);
        }

        #[test]
        fn test_set_valor_costos() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.set_valor_costos(6000, 4000, 2500).is_ok());
            
            assert_eq!(tpfinal.costos.cat_a, 6000);
            assert_eq!(tpfinal.costos.cat_b, 4000);
            assert_eq!(tpfinal.costos.cat_c, 2500);
        }

        #[test]
        fn test_crear_pago_con_socio_nuevo() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Pedro".to_string(), "123456789".to_string(), Categorias::A, None).is_ok()); //aca crea el pago
            
            let pagos = tpfinal.consultar_pagos("123456789".to_string());
            assert!(pagos.is_some());
            let pagos = pagos.unwrap();
            assert_eq!(pagos.len(), 1);

            let pago = &pagos[0];
            assert_eq!(pago.socio.nombre, "Pedro");
            assert_eq!(pago.socio.categoria, Categorias::A);
            assert_eq!(pago.socio.actividad, None);
            assert_eq!(pago.socio.pagos_consecutivos, 0);

        }

        #[test]
        fn test_crear_socio_cat_b() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Pedro Pascal".to_string(), "25687579".to_string(), Categorias::B, Some(Actividad::Rugby)).is_ok());
            
            let socio = tpfinal.retornar_socio(&"25687579".to_string());
            assert!(socio.is_some());
            let socio = socio.unwrap();
            assert_eq!(socio.nombre, "Pedro Pascal");
            assert_eq!(socio.categoria, Categorias::B);
            assert_eq!(socio.actividad, Some(Actividad::Rugby));
            assert_eq!(socio.pagos_consecutivos, 0);
            assert_eq!(tpfinal.socios.len(),1);
            assert_eq!(tpfinal.pagos.len(),1);
        }
        
        #[test]
        fn test_crear_socio_cat_c() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
        
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Leo Messi".to_string(), "12345678".to_string(), Categorias::C,None).is_ok());
    
            let socio = tpfinal.retornar_socio(&"12345678".to_string());
            assert!(socio.is_some());
            let socio = socio.unwrap();
            assert_eq!(socio.nombre, "Leo Messi");
            assert_eq!(socio.categoria, Categorias::C);
            assert_eq!(socio.actividad, None);
            assert_eq!(socio.pagos_consecutivos, 0);
            assert_eq!(tpfinal.socios.len(),1);
            assert_eq!(tpfinal.pagos.len(),1);
        }
        
        #[test]
        fn test_creo_muchos_socios(){ // Pruebo todas las categorias y actividades
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
        
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Juan".to_string(), "12345".to_string(), Categorias::A,None).is_ok());
            assert_eq!(tpfinal.socios.len(),1);
            assert_eq!(tpfinal.pagos.len(),1);
            assert!(tpfinal.priv_crear_socio( "Rami".to_string(), "123456".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
            assert_eq!(tpfinal.socios.len(),2);
            assert_eq!(tpfinal.pagos.len(),2);
            assert!(tpfinal.crear_socio("Leo Messi".to_string(), "12345678".to_string(), Categorias::C,None).is_ok());
            assert_eq!(tpfinal.socios.len(),3);
            assert_eq!(tpfinal.pagos.len(),3);
            assert!(tpfinal.crear_socio("Pedro Pascal".to_string(), "25687579".to_string(), Categorias::B,Some(Actividad::Rugby)).is_ok());
            assert_eq!(tpfinal.socios.len(),4);
            assert_eq!(tpfinal.pagos.len(),4);
            assert!(tpfinal.crear_socio("Paulo Dybala".to_string(), "467589".to_string(), Categorias::B,Some(Actividad::Tenis)).is_ok());
            assert_eq!(tpfinal.socios.len(),5);
            assert_eq!(tpfinal.pagos.len(),5);
            assert!(tpfinal.crear_socio("Leandro Paredes".to_string(), "58595758".to_string(), Categorias::B,Some(Actividad::Basquet)).is_ok());
            assert_eq!(tpfinal.socios.len(),6);
            assert_eq!(tpfinal.pagos.len(),6);
            assert!(tpfinal.crear_socio("Nicolas Otamendi".to_string(), "321456987".to_string(), Categorias::B,Some(Actividad::Hockey)).is_ok());
            assert_eq!(tpfinal.socios.len(),7);
            assert_eq!(tpfinal.pagos.len(),7);
            assert!(tpfinal.crear_socio("Kun Aguero".to_string(), "4569897123".to_string(), Categorias::B,Some(Actividad::Natacion)).is_ok());
            assert_eq!(tpfinal.socios.len(),8);
            assert_eq!(tpfinal.pagos.len(),8);
            assert!(tpfinal.crear_socio("Enzo Fernandez".to_string(), "147369258".to_string(), Categorias::B,Some(Actividad::Paddle)).is_ok());
            assert_eq!(tpfinal.socios.len(),9);
            assert_eq!(tpfinal.pagos.len(),9);
            assert_eq!(tpfinal.consultar_pagos("12345".to_string()).unwrap().len(),1);
            assert_eq!(tpfinal.consultar_pagos("123456".to_string()).unwrap().len(),1);
            assert_eq!(tpfinal.consultar_pagos("123456".to_string()).unwrap().len(),1);
            assert_eq!(tpfinal.consultar_pagos("25687579".to_string()).unwrap().len(),1);
            assert_eq!(tpfinal.consultar_pagos("467589".to_string()).unwrap().len(),1);
            assert_eq!(tpfinal.consultar_pagos("58595758".to_string()).unwrap().len(),1);
            assert_eq!(tpfinal.consultar_pagos("321456987".to_string()).unwrap().len(),1);
            assert_eq!(tpfinal.consultar_pagos("4569897123".to_string()).unwrap().len(),1);
            assert_eq!(tpfinal.consultar_pagos("147369258".to_string()).unwrap().len(),1);
        }

        #[test]
        fn test_pagar_pago() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
            assert!(tpfinal.pagar_pago("2222".to_string(), 3000).is_ok());
            
            let pagos = tpfinal.consultar_pagos("2222".to_string());
            assert!(pagos.is_some());
            let pagos = pagos.unwrap();
            assert_eq!(pagos.len(), 2); // 2 porque creamos el segundo pago luego de pagarlo

            let pago = &pagos[0];
            assert_eq!(pago.socio.nombre, "Fermin");
            assert_eq!(pago.socio.categoria, Categorias::B);
            assert_eq!(pago.socio.actividad, Some(Actividad::Futbol));
            assert_eq!(pago.estado, true);
            /*assert_eq!(pago.socio.pagos_consecutivos, 1); //todavia no sabemos si pago en terminos */
        }

        #[test]
        fn test_pagar_pago_con_descuento() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1688836106000);
            
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());

            // Primer pago
            assert_eq!(tpfinal.pagos[0].vencimiento.dia, 18);
            assert_eq!(tpfinal.pagos[0].vencimiento.mes, 7);
            assert_eq!(tpfinal.pagos[0].vencimiento.anio, 2023);

            assert!(tpfinal.pagar_pago("2222".to_string(), 3000).is_ok());
            
            let pagos = tpfinal.consultar_pagos("2222".to_string());
            assert!(pagos.is_some());
            let pagos = pagos.unwrap();
            assert_eq!(pagos.len(), 2); // 2 porque creamos el segundo pago luego de pagarlo

            let pago = &pagos[0];
            assert_eq!(pago.socio.nombre, "Fermin");
            assert_eq!(pago.socio.categoria, Categorias::B);
            assert_eq!(pago.socio.actividad, Some(Actividad::Futbol));
            assert_eq!(pago.estado, true);
            assert_eq!(pago.socio.pagos_consecutivos, 0); // Todavia no sabemos si pago en terminos 

            // Segundo pago

            assert_eq!(tpfinal.pagos[1].vencimiento.dia, 17);
            assert_eq!(tpfinal.pagos[1].vencimiento.mes, 8);
            assert_eq!(tpfinal.pagos[1].vencimiento.anio, 2023);

            assert!(tpfinal.pagar_pago("2222".to_string(), 3000).is_ok());
            
            let pagos = tpfinal.consultar_pagos("2222".to_string());
            assert!(pagos.is_some());
            let pagos = pagos.unwrap();
            assert_eq!(pagos.len(), 3); // Se crea el tercer pago

            let pago = &pagos[1];
            assert_eq!(pago.socio.nombre, "Fermin");
            assert_eq!(pago.socio.categoria, Categorias::B);
            assert_eq!(pago.socio.actividad, Some(Actividad::Futbol));
            assert_eq!(pago.estado, true);
            assert_eq!(pago.socio.pagos_consecutivos, 1); // Todavia no sabemos si pago en terminos 

            // Tercer pago

            assert_eq!(tpfinal.pagos[2].vencimiento.dia, 16);
            assert_eq!(tpfinal.pagos[2].vencimiento.mes, 9);
            assert_eq!(tpfinal.pagos[2].vencimiento.anio, 2023);

            assert!(tpfinal.pagar_pago("2222".to_string(), 3000).is_ok());
            
            let pagos = tpfinal.consultar_pagos("2222".to_string());
            assert!(pagos.is_some());
            let pagos = pagos.unwrap();
            assert_eq!(pagos.len(), 4); // Se crea el tercer pago

            let pago = &pagos[2];
            assert_eq!(pago.socio.nombre, "Fermin");
            assert_eq!(pago.socio.categoria, Categorias::B);
            assert_eq!(pago.socio.actividad, Some(Actividad::Futbol));
            assert_eq!(pago.estado, true);
            assert_eq!(pago.socio.pagos_consecutivos, 2); 

            // Cuarto pago (el que tiene descuento)

            assert_eq!(tpfinal.pagos[3].vencimiento.dia, 16);
            assert_eq!(tpfinal.pagos[3].vencimiento.mes, 10);
            assert_eq!(tpfinal.pagos[3].vencimiento.anio, 2023);

            assert!(tpfinal.pagar_pago("2222".to_string(), 3000).is_ok());
            
            let pagos = tpfinal.consultar_pagos("2222".to_string());
            assert!(pagos.is_some());
            let pagos = pagos.unwrap();
            assert_eq!(pagos.len(), 5); // Se crea el tercer pago

            let pago = &pagos[3];
            assert_eq!(pago.socio.nombre, "Fermin");
            assert_eq!(pago.socio.categoria, Categorias::B);
            assert_eq!(pago.socio.actividad, Some(Actividad::Futbol));
            assert_eq!(pago.estado, true);
            assert_eq!(pago.socio.pagos_consecutivos, 0);  // Al tercer pago se setea en cero y crea un pago con descuento
        }

        #[test]
        fn test_pagar_pago_vencido() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
    
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
    
            tpfinal.pagos[0].vencimiento.crear_vencimiento(10,1688836106);
    
            assert_eq!(tpfinal.pagos[0].vencimiento.dia, 18);
            assert_eq!(tpfinal.pagos[0].vencimiento.mes, 7);
            assert_eq!(tpfinal.pagos[0].vencimiento.anio, 2023);
    
            // Establecer un timestamp anterior a la fecha de vencimiento del pago
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1690304446000);
    
            assert!(tpfinal.pagar_pago("2222".to_string(), 3000).is_ok());
            assert!(tpfinal.pagos[0].fecha_pago.is_some()); // Verificar que el campo fecha_pago tenga un valor Some
            let fecha = tpfinal.pagos[0].fecha_pago.clone().unwrap(); // Desempaquetar el valor Some
            assert_eq!(fecha.dia, 25);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);
        }

        #[test]
        fn test_fecha_procesar_tiempo() {
            let mut fecha = Fecha::new(0, 0, 0);
            let procesada = fecha.procesar_tiempo(1688671727);

            assert_eq!(procesada.dia, 6);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);
        }

        #[test]
        fn test_fecha_crear_vencimiento(){
            let mut fecha = Fecha::new(6, 7, 2023);
            fecha.crear_vencimiento(10, 1688671727);

            assert_eq!(fecha.dia, 16);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);
        }

        #[test]
        fn test_fecha_es_mayor(){   
            // Retorna true, la fecha si es mayor
            let mut fecha = Fecha::new(0, 0, 0);
            let procesada = fecha.procesar_tiempo(1688671727);
            
            assert_eq!(procesada.dia, 6);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);

            // Le aumento 10 dias
            fecha.crear_vencimiento(10, 1688671727);
            assert_eq!(fecha.dia, 16);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);

            // Creo Fecha 2 para poder comparar
            let fecha2 = Fecha::new(18,8,2023);
            assert_eq!(fecha2.dia, 18);
            assert_eq!(fecha2.mes, 8);
            assert_eq!(fecha2.anio, 2023);
            assert!(fecha2.es_mayor(&fecha));
        }

        #[test]
        fn test_fecha_es_mayor_por_dia(){   
            // Retorna true, la fecha si es mayor
            let mut fecha = Fecha::new(0, 0, 0);

            // Creo Fecha
            let procesada = fecha.procesar_tiempo(1688671727);
            assert_eq!(procesada.dia, 6);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);

            // Le aumento 10 dias
            fecha.crear_vencimiento(10, 1688671727);
            assert_eq!(fecha.dia, 16);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);

            // Creo Fecha 2 para poder comparar
            let fecha2 = Fecha::new(18,7,2023);
            assert_eq!(fecha2.dia, 18);
            assert_eq!(fecha2.mes, 7);
            assert_eq!(fecha2.anio, 2023);
            assert!(fecha2.es_mayor(&fecha));
        }

        #[test]
        fn test_fecha_es_mayor_por_mes(){   
            // Retorna true, la fecha si es mayor
            let mut fecha = Fecha::new(0, 0, 0);

            // Creo Fecha
            let procesada = fecha.procesar_tiempo(1688671727);
            assert_eq!(procesada.dia, 6);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);

            // Le aumento 10 dias
            fecha.crear_vencimiento(10, 1688671727);
            assert_eq!(fecha.dia, 16);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);

            // Creo Fecha 2 para poder comparar
            let fecha2 = Fecha::new(16,8,2023);
            assert_eq!(fecha2.dia, 16);
            assert_eq!(fecha2.mes, 8);
            assert_eq!(fecha2.anio, 2023);
            assert!(fecha2.es_mayor(&fecha));
        }

        #[test]
        fn test_fecha_es_mayor_por_anio(){   
            // Retorna true, la fecha si es mayor
            let mut fecha = Fecha::new(0, 0, 0);

            // Creo Fecha
            let procesada = fecha.procesar_tiempo(1688671727);
            assert_eq!(procesada.dia, 6);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);

            // Le aumento 10 dias
            fecha.crear_vencimiento(10, 1688671727);
            assert_eq!(fecha.dia, 16);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);

            // Creo Fecha 2 para poder comparar
            let fecha2 = Fecha::new(16,7,2024);
            assert_eq!(fecha2.dia, 16);
            assert_eq!(fecha2.mes, 7);
            assert_eq!(fecha2.anio, 2024);
            assert!(fecha2.es_mayor(&fecha));
        }

        #[test]
        fn test_fecha_no_es_mayor_por_anio(){   
            // Retorna false, la fecha no es mayor
            let mut fecha = Fecha::new(0, 0, 0);

            // Creo Fecha
            let procesada = fecha.procesar_tiempo(1688671727);
            assert_eq!(procesada.dia, 6);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);

            // Le aumento 10 dias
            fecha.crear_vencimiento(10, 1688671727);
            assert_eq!(fecha.dia, 16);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);

            // Creo Fecha 2 para poder comparar
            let fecha2 = Fecha::new(16,7,2022);
            assert_eq!(fecha2.dia, 16);
            assert_eq!(fecha2.mes, 7);
            assert_eq!(fecha2.anio, 2022);
            assert!(!fecha2.es_mayor(&fecha));
        }

        #[test]
        fn test_fecha_no_es_mayor_por_mes(){   
            // Retorna false, la fecha no es mayor
            let mut fecha = Fecha::new(0, 0, 0);

            // Creo Fecha
            let procesada = fecha.procesar_tiempo(1688671727);
            assert_eq!(procesada.dia, 6);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);

            // Le aumento 10 dias
            fecha.crear_vencimiento(10, 1688671727);
            assert_eq!(fecha.dia, 16);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);

            // Creo Fecha 2 para poder comparar
            let fecha2 = Fecha::new(16,6,2023);
            assert_eq!(fecha2.dia, 16);
            assert_eq!(fecha2.mes, 6);
            assert_eq!(fecha2.anio, 2023);
            assert!(!fecha2.es_mayor(&fecha));
        }

        #[test]
        fn test_fecha_no_es_mayor_por_dia(){   
            // Retorna false, la fecha no es mayor
            let mut fecha = Fecha::new(0, 0, 0);

            // Creo Fecha
            let procesada = fecha.procesar_tiempo(1688671727);
            assert_eq!(procesada.dia, 6);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);

            // Le aumento 10 dias
            fecha.crear_vencimiento(10, 1688671727);
            assert_eq!(fecha.dia, 16);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);

            // Creo Fecha 2 para poder comparar
            let fecha2 = Fecha::new(15,7,2023);
            assert_eq!(fecha2.dia, 15);
            assert_eq!(fecha2.mes, 7);
            assert_eq!(fecha2.anio, 2023);
            assert!(!fecha2.es_mayor(&fecha));
        }

        #[test]
        fn test_fecha_no_es_mayor_pero_son_iguales() {   
            // Retorna false, la fecha no es mayor sino que son las mismas
            // Osea esta pagando el dia justo en el que se le vence el pago
            let mut fecha = Fecha::new(0, 0, 0);

            // Creo Fecha
            let procesada = fecha.procesar_tiempo(1688671727);
            assert_eq!(procesada.dia, 6);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);

            // Le aumento 10 dias
            fecha.crear_vencimiento(10, 1688671727);
            assert_eq!(fecha.dia, 16);
            assert_eq!(fecha.mes, 7);
            assert_eq!(fecha.anio, 2023);

            // Creo Fecha 2 para poder comparar
            let fecha2 = Fecha::new(16,7,2023);
            assert_eq!(fecha2.dia, 16);
            assert_eq!(fecha2.mes, 7);
            assert_eq!(fecha2.anio, 2023);
            assert!(!fecha2.es_mayor(&fecha));
        }

        #[test]
        fn test_transferir_owner() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            assert!(tpfinal.transferir_owner(account2).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(tpfinal.agregar_direccion_autorizada(account).is_ok());
            assert_eq!(tpfinal.get_owner(), account2);
        }

        #[test]
        fn test_transferir_owner_no_owner() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert_eq!(tpfinal.transferir_owner(account2), Err(Error::NoEsElOwnerError));
        }

        #[test]
        fn test_get_owner() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            assert_eq!(tpfinal.get_owner(), account);
            tpfinal.transferir_owner(account2);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            tpfinal.agregar_direccion_autorizada(account);
            assert_eq!(tpfinal.get_owner(), account2);
        }

        #[test]
        fn test_es_direccion_valida_politica_false() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            tpfinal.intercambiar_politica_de_autorizacion();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(tpfinal.es_direccion_valida());
        }

        #[test]
        fn test_es_direccion_valida_es_owner() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            assert!(tpfinal.es_direccion_valida());
        }

        #[test]
        fn test_es_direccion_valida_esta_autorizado() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            tpfinal.agregar_direccion_autorizada(account2);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(tpfinal.es_direccion_valida());
        }

        #[test]
        fn test_es_direccion_valida_no_autorizado() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert!(!tpfinal.es_direccion_valida());
        }

        #[test]
        fn test_intercambiar_politica_de_autorizacion() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            assert!(tpfinal.get_politica_de_autorizacion());

            tpfinal.intercambiar_politica_de_autorizacion();
            assert!(!tpfinal.get_politica_de_autorizacion());
        }

        #[test]
        fn test_intercambiar_politica_de_autorizacion_no_owner() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert_eq!(tpfinal.intercambiar_politica_de_autorizacion(), Err(Error::NoEsElOwnerError));
        }

        #[test]
        fn test_agregar_direccion_autorizada() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            assert!(tpfinal.agregar_direccion_autorizada(account2).is_ok());
        }

        #[test]
        fn test_agregar_direccion_autorizada_no_owner() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert_eq!(tpfinal.agregar_direccion_autorizada(account2), Err(Error::NoEsElOwnerError));
        }

        #[test]
        fn test_eliminar_direccion_autorizada() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            assert!(tpfinal.agregar_direccion_autorizada(account2).is_ok());
            assert!(tpfinal.eliminar_direccion_autorizada(account2).is_ok());
        }

        #[test]
        fn test_eliminar_direccion_autorizada_no_owner() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            assert!(tpfinal.agregar_direccion_autorizada(account2).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert_eq!(tpfinal.eliminar_direccion_autorizada(account2), Err(Error::NoEsElOwnerError));
        }

        #[test]
        fn test_eliminar_direccion_autorizada_no_econtrada() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000, 3, 10);
            assert_eq!(tpfinal.eliminar_direccion_autorizada(account2), Err(Error::DireccionNoEliminadaError));
        }

        #[test]
        fn test_recaudacion_mensual() {
            //tomamos gran parte de pago con descuento eliminando lo que no es importante
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1688836106000);
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
            // Primer pago primer socio cat B
            assert_eq!(tpfinal.pagos[0].vencimiento.dia, 18);
            assert_eq!(tpfinal.pagos[0].vencimiento.mes, 7);
            assert_eq!(tpfinal.pagos[0].vencimiento.anio, 2023);
            assert!(tpfinal.pagar_pago("2222".to_string(), 3000).is_ok());
            assert!(tpfinal.crear_socio("Santiago".to_string(), "1111".to_string(), Categorias::A, None).is_ok());
            //primer pago segundo socio cat A
            assert!(tpfinal.pagar_pago("1111".to_string(), 5000).is_ok());
            let montos:(u32,u32,u32) = tpfinal.recaudacion_mensual(18,7,2023);
            assert_eq!(montos.0, 5000);
            assert_eq!(montos.1, 3000);
            assert_eq!(montos.2, 0);
        }

        #[test]
            fn test_recaudacion_mensual_todas_las_cat() {
                //tomamos gran parte de pago con descuento eliminando lo que no es importante
                let account = AccountId::from([0x1; 32]);
                ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
                ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1586710800000);
                let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
                // Primer pago primer socio cat B
                assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
                assert_eq!(tpfinal.pagos[0].vencimiento.dia, 22);
                assert_eq!(tpfinal.pagos[0].vencimiento.mes, 4);
                assert_eq!(tpfinal.pagos[0].vencimiento.anio, 2020);
                assert!(tpfinal.pagar_pago("2222".to_string(), 3000).is_ok());
                //primer pago segundo socio cat A
                assert!(tpfinal.crear_socio("Santiago".to_string(), "1111".to_string(), Categorias::A, None).is_ok());
                assert!(tpfinal.pagar_pago("1111".to_string(), 5000).is_ok());
                //primer pago tercer socio cat C
                assert!(tpfinal.crear_socio("Ramiro".to_string(), "3333".to_string(), Categorias::C, None).is_ok());
                assert!(tpfinal.pagar_pago("3333".to_string(), 2000).is_ok());

                //primer pago segundo socio cat A
                assert!(tpfinal.crear_socio("Juan".to_string(), "4444".to_string(), Categorias::A, None).is_ok());
                assert!(tpfinal.pagar_pago("4444".to_string(), 5000).is_ok());
                // Primer pago primer socio cat B
                assert!(tpfinal.crear_socio("Maria".to_string(), "5555".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
                assert!(tpfinal.pagar_pago("5555".to_string(), 3000).is_ok());
                //primer pago tercer socio cat C
                assert!(tpfinal.crear_socio("Carla".to_string(), "6666".to_string(), Categorias::C, None).is_ok());
                assert!(tpfinal.pagar_pago("6666".to_string(), 2000).is_ok());

                let montos:(u32,u32,u32) = tpfinal.recaudacion_mensual(10,4,2020);
                assert_eq!(montos.0, 10000);
                assert_eq!(montos.1, 6000);
                assert_eq!(montos.2, 4000);
            }

        #[test]
        fn test_recaudacion_mensual_fecha_no_valida() {
            //tomamos gran parte de pago con descuento eliminando lo que no es importante
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1688836106000);
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
            // Primer pago primer socio cat B
            assert_eq!(tpfinal.pagos[0].vencimiento.dia, 18);
            assert_eq!(tpfinal.pagos[0].vencimiento.mes, 7);
            assert_eq!(tpfinal.pagos[0].vencimiento.anio, 2023);
            assert!(tpfinal.pagar_pago("2222".to_string(), 3000).is_ok());
            assert!(tpfinal.crear_socio("Santiago".to_string(), "1111".to_string(), Categorias::A, None).is_ok());
            //primer pago segundo socio cat A
            assert!(tpfinal.pagar_pago("1111".to_string(), 5000).is_ok());
            let montos:(u32,u32,u32) = tpfinal.recaudacion_mensual(33,7,2023);
            assert_eq!(montos.0, 0);
            assert_eq!(montos.1, 0);
            assert_eq!(montos.2, 0);
        }

        #[test]
        pub fn test_fecha_febrero_es_bisiesto() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1688836106000);
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            let fecha = Fecha::new(29,2,2020);
            assert!(fecha.es_bisiesto());
        }

        #[test]
        pub fn test_fecha_febrero_no_es_bisiesto() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1688836106000);
            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            let fecha = Fecha::new(28,2,2021);
            assert!(fecha.es_fecha_valida().is_ok());
        }

        #[test]
        pub fn test_socios_pendientes(){
            //Hago un pago en 2020, modifico la fecha a 2023, y tendria que devolver el socio pendiente
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1586710800000);     //Fecha = 12,4,2020

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);            
            assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());

            assert_eq!(tpfinal.pagos[0].vencimiento.dia, 22);
            assert_eq!(tpfinal.pagos[0].vencimiento.mes, 4);
            assert_eq!(tpfinal.pagos[0].vencimiento.anio, 2020);


            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1688836106000); // Fecha = 8,7,2023
            assert!(tpfinal.crear_socio("Rami".to_string(), "1111".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());

            assert_eq!(tpfinal.pagos[1].vencimiento.dia, 18);
            assert_eq!(tpfinal.pagos[1].vencimiento.mes, 7);
            assert_eq!(tpfinal.pagos[1].vencimiento.anio, 2023);

            let mut aux = Fecha::new(0, 0, 0);
            let procesada = aux.procesar_tiempo(1688836106000 / 1000);

            assert_eq!(procesada.dia, 8);
            assert_eq!(procesada.mes, 7);
            assert_eq!(procesada.anio, 2023);

            let tuplas = tpfinal.socios_pendientes();
            assert_eq!(tuplas.len(),1);
            assert_eq!(tuplas[0].0, "Fermin".to_string());
            assert_eq!(tuplas[0].1, "2222".to_string());
        }

        #[test]
        pub fn test_socios_pendientes_retorna_vacio(){
            //Retorna un Vec<> vacio si no hay pagos
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1586710800000); //22,4,2020
            
            let tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            let aux = tpfinal.socios_pendientes();
            assert!(aux.is_empty());
        }

        #[test]
        pub fn test_socios_no_morosos_por_actividad_con_timestamp() {
            //tomamos gran parte de pago con descuento eliminando lo que no es importante
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1586710800000);     //Fecha = 22,4,2020
            assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(1688836106000);     // Fecha = 18,7,2023
            assert!(tpfinal.crear_socio("Juan".to_string(), "3333".to_string(), Categorias::B, Some(Actividad::Tenis)).is_ok());
            assert!(tpfinal.crear_socio("Carlos".to_string(), "4444".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());

            let result = tpfinal.socios_no_morosos_por_actividad("futbol".to_string()).unwrap();
            assert_eq!(result.len(), 1);
            assert_eq!(result[0].0, "Carlos");
            assert_eq!(result[0].1, "4444");
        }

        #[test]
        pub fn test_socios_no_morosos_por_actividad() {
            //tomamos gran parte de pago con descuento eliminando lo que no es importante
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
            assert!(tpfinal.crear_socio("Juan".to_string(), "3333".to_string(), Categorias::B, Some(Actividad::Tenis)).is_ok());
            assert!(tpfinal.crear_socio("Carlos".to_string(), "4444".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());

            let result = tpfinal.socios_no_morosos_por_actividad("futbol".to_string()).unwrap();
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].0, "Fermin");
            assert_eq!(result[0].1, "2222");
            assert_eq!(result[1].0, "Carlos");
            assert_eq!(result[1].1, "4444");
        }

        #[test]
        pub fn test_socios_no_morosos_por_actividad_vacio_agregando() {
            //tomamos gran parte de pago con descuento eliminando lo que no es importante
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            assert!(tpfinal.crear_socio("Fermin".to_string(), "2222".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());
            assert!(tpfinal.crear_socio("Juan".to_string(), "3333".to_string(), Categorias::B, Some(Actividad::Tenis)).is_ok());
            assert!(tpfinal.crear_socio("Carlos".to_string(), "4444".to_string(), Categorias::B, Some(Actividad::Futbol)).is_ok());

            let result = tpfinal.socios_no_morosos_por_actividad("basquet".to_string()).unwrap();
            assert!(result.is_empty());
        }

        #[test]
        pub fn test_socios_no_morosos_por_actividad_vacio_sin_agregar() {
            //tomamos gran parte de pago con descuento eliminando lo que no es importante
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut tpfinal = Tpfinal::new(5000, 3000, 2000,3,10);
            let result = tpfinal.socios_no_morosos_por_actividad("futbol".to_string()).unwrap();
            assert!(result.is_empty());
        }
    }
}
