#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod reportes {
    use tpfinal::TpfinalRef;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.

   
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Actividad {
        Futbol, 
        Basquet, 
        Rugby,
        Hockey, 
        Natacion, 
        Tenis, 
        Paddle,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Retornado cuando se envia una actividad incorrecta
        ActividadError,
        NoEsElOwnerError,
    }



    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
     pub struct Fecha {
        dia:u32,
        mes:u32,
        anio:u32
    }

    impl Fecha{
        fn new(dia: u32, mes: u32, anio: u32) -> Fecha {
            Fecha{ dia, mes, anio }
        }
    }

    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Socio{
        nombre:String,
        dni:String
    }

    impl Socio{
        pub fn new(nombre:String,dni:String) -> Socio{
            Socio{nombre,dni}
        }
    }
    
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
   	pub struct Recaudacion{
      fecha:Fecha,
      monto_cat_a:u32,
      monto_cat_b:u32,
      monto_cat_c:u32
    } 

    impl Recaudacion {
        fn new(fecha:Fecha,monto_cat_a:u32,monto_cat_b:u32,monto_cat_c:u32) -> Recaudacion {
            Recaudacion{fecha,monto_cat_a,monto_cat_b,monto_cat_c}
        }
    }
    
    #[ink(storage)]
    pub struct Reportes {
        /// Stores a single `bool` value on the storage.
        #[cfg(not(test))]
        tpfinal:TpfinalRef,
        owner: AccountId,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Reportes {
        /// Constructor
        #[ink(constructor)]
        #[cfg(not(test))]
        pub fn new(tpfinal: TpfinalRef) -> Self {
            let owner = Self::env().caller();
            Self { tpfinal,owner }
        }

        #[cfg(test)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            Self { owner }
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
        
        /// Retorna los socios con pagos vencidos.
        #[ink(message)]
        #[cfg(not(test))]
        pub fn socios_pendientes(&self) -> Vec<Socio>{
            self.priv_socios_pendientes()
        } 
        

        #[cfg(not(test))]
        fn priv_socios_pendientes(&self) -> Vec<Socio>{
            let tuplas = self.tpfinal.socios_pendientes();
            let socios: Vec<Socio> = tuplas.into_iter().map(|(nombre, dni)| Socio::new(nombre, dni)).collect();
            return socios
        }

        #[cfg(test)]
        fn valor_socios_pendientes(&self) -> Vec<(String, String)>  {
                vec![
                    ("rami".to_string(), "1111".to_string()),
                    ("fermin".to_string(), "2222".to_string()),
                    ("santi".to_string(), "3333".to_string()),
                ]
        }

        #[cfg(test)]
        fn socios_pendientes(&self)  -> Vec<Socio> {
                let tuplas =  self.valor_socios_pendientes();
                let socios: Vec<Socio> = tuplas.into_iter().map(|(nombre, dni)| Socio::new(nombre, dni)).collect();
                return socios;
            }

        /// Recibe una actividad como String.
        /// Retorna todos los socios no morosos que pueden ir a la actividad recibida.
        /// Si la actividad recibida no existe retorna error.
        #[ink(message)]
        #[cfg(not(test))]
        pub fn socios_no_morosos_por_actividad(&self, actividad:String) -> Result<Vec<Socio>>{
            self.priv_socios_no_morosos_por_actividad(actividad)
        }

        #[cfg(not(test))]
        fn priv_socios_no_morosos_por_actividad(&self, actividad:String) -> Result<Vec<Socio>>{
            
            let tuplas = self.tpfinal.socios_no_morosos_por_actividad(actividad);
            if tuplas.is_none(){return Err(Error::ActividadError)}
            else {return Ok(tuplas.unwrap().into_iter().map(|(nombre, dni)| Socio::new(nombre, dni)).collect())}
        }

        #[cfg(test)] 
        fn valores_socios_no_morosos_por_actividad(&self, actividad: String) -> Option<Vec<(String,String)>> {
                match actividad.as_str() {
                "basquet"  => Some(vec![("rami".to_string(), "1111".to_string())]),
                "futbol" => Some(vec![("fermin".to_string(), "2222".to_string())]),
                "tenis" => Some(vec![("santi".to_string(), "3333".to_string())]),
                _ => None,
            }
        }

        #[cfg(test)]
        fn socios_no_morosos_por_actividad(&self, actividad: String) -> Result<Vec<Socio>> {
                let tuplas =   self.valores_socios_no_morosos_por_actividad(actividad);
                if tuplas.is_none(){return Err(Error::ActividadError)}
                else {return Ok(tuplas.unwrap().into_iter().map(|(nombre, dni)| Socio::new(nombre, dni)).collect())}
        }
        


        /// Se envia una fecha y retorna el recaudamiento de ese mes por cada categoria. Si hay algun error retorna 0 en todas las categorias.
        #[ink(message)]
        #[cfg(not(test))]
        pub fn recaudacion_mensual(&self,dia:u32,mes:u32,anio:u32) -> Recaudacion{
            self.priv_recaudacion_mensual(dia,mes,anio)
        }

        //Se envia una fecha, y devuelve el recaudamiento de ese mes, por cada categoria. Si hay algun error, retorna 0 en todas las categorias
        #[cfg(not(test))]
        fn priv_recaudacion_mensual(&self,dia:u32,mes:u32,anio:u32) -> Recaudacion {
            let aux = self.tpfinal.recaudacion_mensual(dia,mes,anio);
            let fecha = Fecha::new(dia,mes,anio);
            return Recaudacion::new(fecha,  aux.0,aux.1,aux.2)
        }

        #[cfg(test)]
        fn valores_recaudacion_mensual(&self, dia: u32, mes: u32, anio: u32) -> (u32, u32, u32) {
            match (dia, mes, anio) {
                (1, 1, 2023) => (5000, 3000, 2000),
                _ => (0, 0, 0),
            }
        }

        #[cfg(test)]
        fn recaudacion_mensual(&self, dia: u32, mes: u32, anio: u32) -> Recaudacion {
            let tuplas =   self.valores_recaudacion_mensual(dia, mes, anio);
            let fecha = Fecha::new(dia,mes,anio);
            let recaudacion = Recaudacion::new(fecha,tuplas.0,tuplas.1,tuplas.2);
            return recaudacion;
        }

        
    }


    
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use super::*;

         #[test]
        fn test_transferir_owner() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut contrato = Reportes::new();
            assert!(contrato.transferir_owner(account2).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert_eq!(contrato.get_owner(), account2);
        }

        #[test]
        fn test_transferir_owner_no_owner() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut contrato: Reportes = Reportes::new();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert_eq!(contrato.transferir_owner(account2), Err(Error::NoEsElOwnerError));
        }

        #[test]
        fn test_get_owner() {
            let account = AccountId::from([0x1; 32]);
            let account2 = AccountId::from([0x2; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);

            let mut contrato: Reportes = Reportes::new();
            assert_eq!(contrato.get_owner(), account);
            contrato.transferir_owner(account2);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account2);
            assert_eq!(contrato.get_owner(), account2);
        }

        #[test]
        fn test_socios_pendientes() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let contrato = Reportes::new();

           let socios = contrato.socios_pendientes();

            assert_eq!(socios.len(), 3);
            assert_eq!(socios[0].nombre, "rami");
            assert_eq!(socios[0].dni, "1111");
            assert_eq!(socios[1].nombre, "fermin");
            assert_eq!(socios[1].dni, "2222");
            assert_eq!(socios[2].nombre, "santi");
            assert_eq!(socios[2].dni, "3333");

        }


        #[test]
        fn test_socios_no_morosos_por_actividad() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let contrato = Reportes::new();

            let socios = contrato.socios_no_morosos_por_actividad("futbol".to_string());

            assert_eq!(socios.is_ok(), true);
            let socios = socios.unwrap();
            assert_eq!(socios.len(), 1);
            assert_eq!(socios[0].nombre, "fermin");
            assert_eq!(socios[0].dni, "2222");

            // llamamos a la funcion para que de error
            let socios = contrato.socios_no_morosos_por_actividad("error".to_string());

            // retorna un error
            assert_eq!(socios.is_err(), true);
            assert_eq!(socios.unwrap_err(), Error::ActividadError);
        }

        #[test]
        fn test_recaudacion_mensual() {
            let account = AccountId::from([0x1; 32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
            let contract = Reportes::new();

            // fecha valida
            let recaudacion = contract.recaudacion_mensual(1,1,2023);

            // Assert the expected result
            assert_eq!(recaudacion.fecha.dia, 1);
            assert_eq!(recaudacion.fecha.mes, 1);
            assert_eq!(recaudacion.fecha.anio, 2023);
            assert_eq!(recaudacion.monto_cat_a, 5000);
            assert_eq!(recaudacion.monto_cat_b, 3000);
            assert_eq!(recaudacion.monto_cat_c, 2000);

            // Llamada con una fecha invalida (invalida para el test)
             let recaudacion = contract.recaudacion_mensual(1,32,2023);

            // Assert the expected result (all zeros)
            assert_eq!(recaudacion.fecha.dia, 1);
            assert_eq!(recaudacion.fecha.mes, 32);
            assert_eq!(recaudacion.fecha.anio, 2023);
            assert_eq!(recaudacion.monto_cat_a, 0);
            assert_eq!(recaudacion.monto_cat_b, 0);
            assert_eq!(recaudacion.monto_cat_c, 0);
        }
    }

}
  
