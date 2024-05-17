/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema2;

import PaqueteLectura.GeneradorAleatorio;

/**
 *
 * @author alumnos
 */
public class Ej02Practica2 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        GeneradorAleatorio.iniciar();
        int DF = 15,DL = 0, aux = -1;
        Persona [] Personas = new Persona[DF];
        do {
            if (aux != 0) {
                Personas[DL]=new Persona();
                System.out.println("El nombre de la persona: ");
                Personas[DL].setNombre(GeneradorAleatorio.generarString(5));
                System.out.println(Personas[DL].getNombre());
                
                System.out.println("Su Dni: ");
                Personas[DL].setDNI(GeneradorAleatorio.generarInt(2000));
                System.out.println(Personas[DL].getDNI());
                
                System.out.println("La edad: ");
                Personas[DL].setEdad(GeneradorAleatorio.generarInt(10));
                System.out.println(Personas[DL].getEdad());
                
                System.out.println();
                aux = Personas[DL].getEdad();
                DL++;
            }
        } while ((DL < DF) && (aux != 0));
        int i;
        for (i=0;i<DL;i++)
            System.out.println(Personas[i].toString());
    }
    
}
