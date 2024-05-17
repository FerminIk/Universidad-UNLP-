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
public class Triangulo {
    
 /** Punto 1 Hecho por el turno D, la clase original no tenia ningun contenido
 */
    
    private double Lado1;
    private double Lado2;
    private double Lado3;
    private String ColorInterno;
    private String ColorLinea;

    public Triangulo(double Lado1, double Lado2, double Lado3, String ColorInterno, String ColorLinea) {
        this.Lado1 = Lado1;
        this.Lado2 = Lado2;
        this.Lado3 = Lado3;
        this.ColorInterno = ColorInterno;
        this.ColorLinea = ColorLinea;
    }
    
    
    
    public Triangulo() {
        
    }

    public void setLado1(double Lado1) {
        this.Lado1 = Lado1;
    }

    public void setLado2(double Lado2) {
        this.Lado2 = Lado2;
    }

    public void setLado3(double Lado3) {
        this.Lado3 = Lado3;
    }

    public void setColorInterno(String ColorInterno) {
        this.ColorInterno = ColorInterno;
    }

    public void setColorLinea(String ColorLinea) {
        this.ColorLinea = ColorLinea;
    }

    public double getLado1() {
        return Lado1;
    }

    public double getLado2() {
        return Lado2;
    }

    public double getLado3() {
        return Lado3;
    }

    public String getColorInterno() {
        return ColorInterno;
    }

    public String getColorLinea() {
        return ColorLinea;
    }
    
    public double CalcularPerimetro(double Lado1,double Lado2,double Lado3) {
        
        return Lado1+Lado2+Lado3;
    }
    
    public double CalcularPerimetro() {
        
        return this.Lado1+this.Lado2+this.Lado3;
    }
    
    public double CalcularArea() {
        return  Math.sqrt(CalcularS()*(CalcularS() -this.Lado1)*(CalcularS()-this.Lado2)*(CalcularS()-this.Lado3));
    }
    
    private double CalcularS() {
        
       return  CalcularPerimetro() / 2;
    }
    
}
