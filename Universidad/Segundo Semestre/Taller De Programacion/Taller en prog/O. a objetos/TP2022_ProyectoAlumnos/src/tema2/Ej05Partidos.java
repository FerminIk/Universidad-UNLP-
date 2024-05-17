/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
package tema2;

/**
 *
 * @author Mar
 */
import PaqueteLectura.GeneradorAleatorio;
import PaqueteLectura.Lector;

public class Ej05Partidos {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        GeneradorAleatorio.iniciar();
        int DF = 20;
        Partido [] p = new Partido[DF];
        int DL=-1,i=0;
        do {
            DL++;
            p[DL] = new Partido();
            System.out.println("Introduzca el equipo local");
            p[DL].setLocal(Lector.leerString());
            System.out.println("Introduzca el equipo visitante");
            p[DL].setVisitante(Lector.leerString());
            if (!p[DL].getLocal().equals("ZZZ") && !p[DL].getVisitante().equals("ZZZ")) {
                p[DL].setGolesLocal(GeneradorAleatorio.generarInt(5));
                System.out.println(p[DL].getGolesLocal());
                p[DL].setGolesVisitante(GeneradorAleatorio.generarInt(5));
                System.out.println(p[DL].getGolesVisitante());
            }
        } while (!p[DL].getLocal().equals("ZZZ") && !p[DL].getVisitante().equals("ZZZ") && i<20);
        int cant = 0, cantGol = 0;
        for (i=0;i<DL;i++){
            System.out.println("{EQUIPO-LOCAL "+ p[i].getGolesLocal() +" VS EQUIPO-VISITANTE "+ p[i].getGolesVisitante());
            if (p[i].hayGanador()) {
                if (p[i].getGanador().equals("River")) {
                    cant++;
                }
        }
            if (p[i].getLocal().equals("Boca")){
                cantGol = cantGol + p[i].getGolesLocal();
            }
        }
        if (cant != 0) {
            System.out.println("La cantidad de partidos que gano river fue de: "+cant);
        } else {
             System.out.println("River no gano  ningun partido");
        }
        if (cantGol != 0) {
             System.out.println("La cantidad de goles que hizo boca de local fue de: "+cantGol);
        } else {
             System.out.println("Boca no hizo goles");
        }
    }
    
}
