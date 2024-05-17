/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
package tema3;

import PaqueteLectura.GeneradorAleatorio;
import PaqueteLectura.Lector;

/**
 *
 * @author Mar
 */
public class Ej03puntoB {

    /**
     * @param args the command line arguments
     */
    
    
    public static void main(String[] args) {
        GeneradorAleatorio.iniciar();
        Libro l[] = new Libro[5];
        Autor a = new Autor();
        boolean espacio = true;
        int i =  0;
        
        l[i] = new Libro();
        System.out.println("Introduzca un titulo");
        l[i].setTitulo(Lector.leerString());
         while (!l[i].getTitulo().equals("ZZZ")) {
            
            if (espacio) {
  
                a.setNombre(GeneradorAleatorio.generarString(5));

                a.setBiografia(GeneradorAleatorio.generarString(5));

                a.setOrigen(GeneradorAleatorio.generarString(5));


                l[i].setAutor(a);


                l[i].setEditorial(GeneradorAleatorio.generarString(5));

                l[i].setAñoEdicion(GeneradorAleatorio.generarInt(5));


                l[i].setISBN(GeneradorAleatorio.generarString(5));

   
                l[i].setPrecio(GeneradorAleatorio.generarDouble(5));


                i++;
                l[i] = new Libro();
                System.out.println("Introduzca un titulo");
                l[i].setTitulo(Lector.leerString());
            }
        }
        
        Estante e =  new Estante(5);
         
        e.setDimL(i);
        e.setLibros(l);
        
        e.imprimir();
        
        Libro lAux;
        
        lAux = e.buscarLibro("mujercitas");
        if (lAux != null) {
            a = lAux.getAutor();
            System.out.println(a.getNombre());
        } else {
            System.out.println("No Estaba");
        }
             
        
    }
    
}
