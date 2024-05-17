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
public class Ej05Restaurante {

    /**
     * @param args the command line arguments
     */
    public static void main(String args[]) {
        int DF = 5,DC = 4;
        int matriz [][] = new int [DF][DC];
        int i,j,cant,total;
        for (i=0;i<DF;i++) {
            System.out.println("cliente numero "+i);
            for (j=0;j<DC;j++) {
                System.out.println("Introduzca la clasificacion para la categoria "+j+" (del 1 al 10) ");
                matriz[i][j] = Lector.leerInt();
            }
        }
        for (i=0;i<DC;i++) {
            total = 0;
            cant = 0;
            for (j=0;j<DF;j++) {
                total = total + matriz[j][i];
                cant++;
            }
        double res = (double) total/cant;
        System.out.println("Para la categoria "+i+" el promedio es: "+res);
        }   
    }
}
