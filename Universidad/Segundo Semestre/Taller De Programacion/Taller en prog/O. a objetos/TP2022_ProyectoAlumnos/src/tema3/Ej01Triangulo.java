/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema3;

/**
 *
 * @author alumnos
 */

import PaqueteLectura.Lector;
public class Ej01Triangulo {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        // TODO code application logic here
        Triangulo t = new Triangulo();
        System.out.println("Introduzca el largo de la base ");
        t.setLado1(Lector.leerDouble());
        System.out.println("Introduzca el largo del segundo lado ");
        t.setLado2(Lector.leerDouble());
        System.out.println("Introduzca el largo del segundo lado ");
        t.setLado3(Lector.leerDouble());
        
        System.out.println("Introduzca el color interno del triangulo ");
        t.setColorInterno(Lector.leerString());
        
        System.out.println("Introduzca el color de sus lineas ");
        t.setColorLinea(Lector.leerString());
        
        System.out.println("El perimetro del triangulo es: "+t.CalcularPerimetro());
        System.out.println("El area del triangulo es: "+t.CalcularArea());
    }
    
}
