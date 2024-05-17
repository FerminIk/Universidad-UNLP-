

package tema2;

import PaqueteLectura.GeneradorAleatorio;
import PaqueteLectura.Lector;

/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
public class Ej04Lector {

    /**
     * @param args the command line arguments
     */
    
    
    public static void main(String[] args) {
        int DF = 5, DC = 8;
    Persona [][] matriz = new Persona [DF][DC];
    String aux;
    int i, j;
    for (i=0;i<5;i++){
        for (j=0;j<8;j++) {
            matriz [i][j] = null;
        }
        
    }
       
    i = 0;
    do {
        j = 0;
        System.out.println("Introduzca el nombre de la persona");
        aux = Lector.leerString();
        while (!aux.equals("zzz")  && j < 8) {
            matriz [i][j] = new Persona();
            matriz [i][j].setNombre(aux);
            matriz [i][j].setDNI(GeneradorAleatorio.generarInt(100));
            matriz[i][j].setEdad(GeneradorAleatorio.generarInt(25));
    
            System.out.println("Introduzca el nombre de la persona");
            aux = aux = Lector.leerString();;
            
            j++;
        }
        i++;
    } while(!aux.equals("zzz") && i < 5);
    
    int cant = 0;
    for (i=0;i<5;i++) {
        for (j=0;j<8;j++) {
            System.out.println(matriz[i][j]);
            cant++;
        }
    }
    System.out.println(cant);
    }
    
}
