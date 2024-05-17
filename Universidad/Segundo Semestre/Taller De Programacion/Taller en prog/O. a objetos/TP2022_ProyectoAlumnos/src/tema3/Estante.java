/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package tema3;
 
public class Estante {
    private Libro[] libros;
    private int dimL;
    private int dimF;
 
    public Estante(int N) {
        this.dimL = 0;
        this.dimF = N;
        this.libros = new Libro[N];
    }
    
    public Estante() {
        this.dimL = 0;
        this.dimF = 20;
        this.libros = new Libro[dimF];
    }
    
    
    public Libro[] getLibros() {
        return libros;
    }
 
    public void setLibros(Libro[] libros) {
        this.libros = libros;
    }
 
    public int getDimL() {
        return dimL;
    }
 
    public void setDimL(int dimL) {
        this.dimL = dimL;
    }
 
    public int getDimF() {
        return dimF;
    }
 
    public void setDimF(int dimF) {
        this.dimF = dimF;
    }
 
    
 
    public int cantLibros() {
        return this.getDimL();
    }
 
    public boolean lleno() {
        return this.getDimL() != this.getDimF();
    }
 
    public void IncrementarDimL() {
        this.setDimL(this.getDimL() + 1);
    }
 
    public void agregarLibro(Libro libro) {
        if (dimL < dimF) {
            this.libros [dimL] = new Libro();
            this.libros[dimL] = libro;
            this.dimL++;
        }
    }
 
    public Libro buscarLibro(String titulo) {
        int i;
        Libro auxLibro = null; // INICIALIZAR TITULO CON 'ZZZ'
        for (i=0; i<dimL; i++)
            if (libros[i].getTitulo().equals(titulo))
                auxLibro = libros[i];
        return auxLibro;
    }
    
    public void imprimir () {
        int i;
        for (i=0;i<dimL;i++) {
            System.out.println(libros[i].getTitulo());
        }
    }
}
