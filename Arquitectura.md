Que estoy desarrollando:
Motor de Pagos tipo Stripe

Descripcion del motor

Es un motor headless (sin interfaz visual de usuario tradicional): Funciona a través de una API HTTP donde otros servicios le mandan órdenes crudas ("crea una cuenta", "transfiere X dinero de A a B").

Es el guardián de la contabilidad: Su único propósito en la vida es garantizar que las cuentas existan, que las reglas de negocio (fondos suficientes, cuentas activas) se cumplan y que el dinero nunca se cree ni desaparezca de la nada (consistencia contable estricta de doble entrada o transferencia atómica).

Maneja concurrencia de alta intensidad: Su trabajo duro ocurre cuando recibe cientos de peticiones al mismo tiempo y debe decidir qué transacción pasa y cuál se bloquea por conflicto de saldo.

Dominio:

Entidades:
Cuenta tienen:
-id
-cuenta
-estado_cuenta
-nombre
-saldo

Transacion tiene:
-id
-fecha
-cuenta origen 
-cuenta de destino
-estado

catalogo_estado_transacion
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed { reason: String }, // Encapsula el porqué falló
}

catalogo_estado_cuenta
#[derive(Debug, Clone, PartialEq)]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}


Observaciones iniciales : Tengo que decidirme como manejar el dinero en el backend lo normal seria trabajarlo en decimal , pero existen librerias para ello.

Tengo que tener como una idea del neogcio en si para poder, construir el dominio .

Obviamente los estados de cualquier entidad voy a aprovechar al maximo los enums de rust que son muy robustos y evitan que mi aplicacion sea debil

Lo que puede salir mal:
-No puede existir cuentas con saldo negativo
-mi pregunta es aqui si es un motor de pago por que deberia saber si la cuenta esta activa o bloqueada?
-Una cuenta no se puede transferir a la misma cuenta


mas o menos seria este el flujo

Validación de existencia: ¿Existen la cuenta de origen y la cuenta de destino en la base de datos? (Si el ID no existe, se rechaza inmediatamente).

Validación de autorreferencia: ¿Es la cuenta de origen exactamente la misma que la de destino? (Si es así, se rechaza: no puedes transferirte a ti mismo en una misma orden de transferencia directa).

Validación de estado (estado_cuenta): ¿Están ambas cuentas en un estado Active? Si alguna de las dos está Frozen (congelada) o Closed (cerrada), la transacción se aborta de inmediato.

Validación de fondos (Saldo): ¿Tiene la cuenta de origen el saldo suficiente para cubrir el monto de la transacción?

Definiendo scopes:

Selecionado backend en centavos 

