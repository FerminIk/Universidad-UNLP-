/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package tema1;

//Paso 1. importar la funcionalidad para generar datos aleatorios

import PaqueteLectura.GeneradorAleatorio;
import PaqueteLectura.Lector;
public class Ej03Matrices {

    public static void main(String[] args) {
        GeneradorAleatorio.iniciar();
	    //Paso 2. iniciar el generador aleatorio   
        int DF=5,DC = 5;
        int [] vector = new int [DF];
	int [][] matriz = new int [DF] [DC];
        //Paso 3. definir la matriz de enteros de 5x5 e iniciarla con nros. aleatorios 
        int i, j;
        for (i=0;i<DF;i++){
            for (j=0;j<DC;j++)
                matriz[i][j] = GeneradorAleatorio.generarInt(30);
        }
        //Paso 4. mostrar el contenido de la matriz en consola
        for (i=0;i<DF;i++){
            System.out.println("Fila: " +i);
            for (j=0;j<DC;j++)
                System.out.println(matriz[i][j]);
        }
        //Paso 5. calcular e informar la suma de los elementos de la fila 1
        int total=0;
        for (j=0;j<DF;j++)
            total = total + matriz[0][j];
        System.out.println("Total fila 1 "+  total);
        //Paso 6. generar un vector de 5 posiciones donde cada posición j contiene la suma de los elementos de la columna j de la matriz. 
        //        Luego, imprima el vector.
        for (i=0;i<DF;i++) {
        total=0;
            for (j=0;j<DF;j++)
            total = total + matriz[i][j];
        vector[i]=total;
        System.out.println("Total fila  "+i+" "+  vector[i]);
        }
        //Paso 7. lea un valor entero e indique si se encuentra o no en la matriz. En caso de encontrarse indique su ubicación (fila y columna)
        //   y en caso contrario imprima "No se encontró el elemento".
        System.out.println("Introduzca el valor a buscar");
        int buscar = Lector.leerInt();
        boolean b = true;
        for (i=0;i<DF;i++){
            for (j=0;j<DC;j++) {
                if (buscar == matriz[i][j] && b) {
                    b = false;
                    System.out.println("Se encontro el valor "+ buscar + " en la posicion: fila "+i+" columna: "+j);
                }
            }
        }
        if (b)
            System.out.println("No se encontro el valor buscado");
    }
}
