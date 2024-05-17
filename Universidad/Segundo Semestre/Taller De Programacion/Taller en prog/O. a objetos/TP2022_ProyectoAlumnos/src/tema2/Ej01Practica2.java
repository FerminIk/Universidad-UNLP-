/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema2;

import PaqueteLectura.Lector;
/**
 *
 * @author alumnos
 */
public class Ej01Practica2 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        Persona p = new Persona();
        System.out.println("Introduzca el nombre");
        p.setNombre(Lector.leerString());
        System.out.println("Introduzca el dni");
        p.setDNI(Lector.leerInt());
        System.out.println("Introduzca el edad");
        p.setEdad(Lector.leerInt());
        System.out.println(p.toString());
       
        
        
    }
    
}
