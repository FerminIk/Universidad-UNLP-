/*
Clase Libro a la cual se agregaron constructores. 
 */
package tema3;

/**
 *
 * @author vsanz
 */
public class Libro {
   private String titulo;
   private Autor autor; 
   private String editorial;
   private int añoEdicion;
   private String ISBN; 
   private double precio; 
     
    
    public Libro(  String unTitulo,  String unaEditorial, 
    int unAñoEdicion,  Autor unAutor, String unISBN, double unPrecio){
         titulo = unTitulo;
         autor = unAutor;
         editorial = unaEditorial;
         añoEdicion= unAñoEdicion;
         ISBN =  unISBN;
         precio = unPrecio;
    }
    
    
    public Libro(){
   
    }
        
    public String getTitulo(){
        return titulo;
    }
  
    public Autor getAutor(){
        return this.autor;
    }
    
    public String getEditorial(){
        return editorial;
    }
    public int getAñoEdicion(){
        return añoEdicion;
    }
   
    public String getISBN(){
        return ISBN;
    } 
    public double getPrecio(){
        return precio;
    }
   
    public void setTitulo(String unTitulo){
        titulo = unTitulo;
    }
    
    public void setAutor(Autor unAutor){
        autor = unAutor;
    }
   
    public void setEditorial(String unaEditorial){
         editorial = unaEditorial;
    }
    public void setAñoEdicion(int unAño){
         añoEdicion = unAño;
    }
    
    public void setISBN(String unISBN){
         ISBN=unISBN;
    } 
    public void setPrecio(double unPrecio){
         precio=unPrecio;
    }
   
    
   @Override
    public String toString(){
        String aux;
        aux="titulo: "+ titulo + " por " + autor.toString() + " - " + añoEdicion + " - " + " ISBN: " + ISBN +" precio "+precio; 
       return ( aux);
    }
        
}
