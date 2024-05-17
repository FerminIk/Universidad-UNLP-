/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
package tema3;

/**
 *
 * @author Mar
 */

import PaqueteLectura.GeneradorAleatorio;
import PaqueteLectura.Lector;

public class Ej04Hotelcito {

    /**
     * @param args the command line arguments
     */
    
    
    
    public static void main(String[] args) {
        GeneradorAleatorio.iniciar();
        cliente c = new cliente();
        Hotel hotel= new Hotel(10);
        
        
        hotel.inicializar();
        
        System.out.println("Introduzca los datos del cliente: ");
        c.setNombre(Lector.leerString());
        while (!c.getNombre().equals("ZZZ")) {
            System.out.println("Introduzca habitacion donde quiere ir");
            int nro =  Lector.leerInt();
            hotel.IngresarCliente(c, nro);
            c = new cliente();
            System.out.println("Introduzca los datos del cliente: ");
            c.setNombre(Lector.leerString());
            
        }
        
        hotel.imprimir();
        hotel.aumentarPrecio(3000);
        System.out.println();
        System.out.println();
        hotel.imprimir();
    }
    
}
