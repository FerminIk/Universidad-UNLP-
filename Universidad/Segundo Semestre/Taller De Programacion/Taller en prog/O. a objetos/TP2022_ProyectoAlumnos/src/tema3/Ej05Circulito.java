/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
package tema3;

/**
 *
 * @author Mar
 */

import PaqueteLectura.Lector;
public class Ej05Circulito {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        Circulo c = new Circulo();
        System.out.println("Introduzca  el radio del circulo: ");
        c.setRadio(Lector.leerDouble());
        System.out.println("Introduzca  el color del circulo: ");
        c.setColorInterno(Lector.leerString());
        System.out.println("Introduzca  el color de la linea: ");
        c.setColorLinea(Lector.leerString());
        
        double area,perimetro;
        
        area = c.calcularArea();
        perimetro = c.calcularPerimetro();
        
        System.out.println(" El perimetro del circulo es: "+Math.round(perimetro*100)/100d);
        System.out.println(" El area del circulo es: "+Math.round(area*100)/100d);
    }
    
}
