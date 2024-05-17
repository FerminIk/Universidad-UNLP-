/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Other/File.java to edit this template
 */
package tema1;

/**
 *
 * @author Mar
 */
import PaqueteLectura.Lector;
public class Ej04Edificio {

    /**
     * @param args the command line arguments
     */
    
    public static void main(String args[]) {
        int DF = 8, DC = 4;
        int [][] edificio = new int [DF] [DC];
        int i, j, piso, oficina;
        for (i=0;i<8;i++) {
            for (j=0;j<4;j++)
                edificio[i][j] = 0;
        }
        System.out.println("Introduzca el piso en el que trabajas: ");
        piso = Lector.leerInt()-1;
        System.out.println("Introduzca la oficina en el que trabajas: ");
        oficina = Lector.leerInt()-1;
        while (piso != 9) {
            edificio [piso][oficina]++;
            System.out.println("Introduzca el piso en el que trabajas: ");
            piso = Lector.leerInt();
            System.out.println("Introduzca la oficina en el que trabajas: ");
            oficina = Lector.leerInt();
        } 
         for (i=0;i<8;i++) {
            for (j=0;j<4;j++)
                if (edificio [i][j] == 0)
                    System.out.println("piso "+i+"Oficina "+j+" vacia.");
                else
                    System.out.println("Cantidad de personas en el piso "+i+ " oficina "+j+ " de: "+edificio[i][j]);
        }
    }
}
