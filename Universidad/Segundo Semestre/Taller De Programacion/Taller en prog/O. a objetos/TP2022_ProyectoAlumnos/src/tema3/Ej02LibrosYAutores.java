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

import PaqueteLectura.GeneradorAleatorio;
public class Ej02LibrosYAutores {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        GeneradorAleatorio.iniciar();
        Libro libro = new Libro();
        Autor autor = new Autor();
        
        autor.setNombre(GeneradorAleatorio.generarString(10));
        autor.setBiografia(GeneradorAleatorio.generarString(5));
        autor.setOrigen(GeneradorAleatorio.generarString(3));
        
        libro.setTitulo(GeneradorAleatorio.generarString(8));
        libro.setAutor(autor);
        libro.setEditorial(GeneradorAleatorio.generarString(1));
        libro.setAñoEdicion(GeneradorAleatorio.generarInt(4));
        libro.setISBN(GeneradorAleatorio.generarString(8));
        libro.setPrecio(GeneradorAleatorio.generarDouble(5));
        
        System.out.println(libro.toString());
        System.out.println(autor.toString());
        
        
        
    }
    
}
