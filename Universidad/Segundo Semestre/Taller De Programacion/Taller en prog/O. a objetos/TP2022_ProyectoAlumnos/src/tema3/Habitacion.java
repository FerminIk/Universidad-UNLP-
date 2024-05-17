/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package tema3;

/**
 *
 * @author Mar
 */

import PaqueteLectura.GeneradorAleatorio;
public class Habitacion {
    private double CostoPorNoche;
    private boolean Ocupado;
    private cliente Cliente;


    public Habitacion() {
        GeneradorAleatorio.iniciar();
        this.CostoPorNoche = GeneradorAleatorio.generarDouble(6000)+2000;
        this.Ocupado = false;
    }

    public void setCostoPorNoche(double CostoPorNoche) {
        this.CostoPorNoche = CostoPorNoche;
    }

    public void setOcupado(boolean Ocupado) {
        this.Ocupado = Ocupado;
    }

    public void setCliente(cliente Cliente) {
        if (!isOcupado()){
            this.Cliente = Cliente;
            this.Ocupado = true;
        } else {
            System.out.println("Habitacion ocupada");
        }
        
    }

    public double getCostoPorNoche() {
        return CostoPorNoche;
    }

    public boolean isOcupado() {
        return Ocupado;
    }

    public cliente getCliente() {
        return Cliente;
    }
    
    
}
