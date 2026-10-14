
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryDirection {
    Debit,
    Credit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed { reason: String },
}

#[derive(Debug, Clone)]
pub struct Entry {
    id: u64,
    account_id: u64,
    amount_cents: u64,
    direction: EntryDirection,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    id: u64,
    status: TransactionStatus,
    entries: Vec<Entry>,
    created_at: i64, // Unix timestamp en milisegundos o UTC
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

#[derive(Debug, Clone)]
pub struct Account {
    id: u64,// este id esta referenciado a Entru account id sino que la base de datos todavia no esta creada y esta es una representacion a nivel de memoria
    status: AccountStatus,
    balance_cents: u64, // El saldo acumulado en la base de datos
}

impl Transaction {
    fn is_balanced(&self) -> Result<bool, TransactionError>{
        //self.entries.iter().map(|e|e.amount_cents).sum::<u64>() == 0
        let mut sum_debit = 0u64;
        let mut sum_credit = 0u64;

        for entry in &self.entries  {
            match entry.direction  {
                EntryDirection::Debit => sum_debit = sum_debit.checked_add(entry.amount_cents).ok_or(TransactionError::ArithmeticOverflow)?,
                EntryDirection::Credit => sum_credit =sum_credit.checked_add(entry.amount_cents).ok_or(TransactionError::ArithmeticOverflow)?,
            }
        };
        Ok(sum_credit == sum_debit)
    }
    // Modificar la firma de la funcion y definir errores posibles

    fn new(id:u64, entries : Vec<Entry> , created_at : i64)-> Result<Transaction, TransactionError> {
        //Validaciones o las famosas clausuras de guarda
        // definir como se va a hacer para el created_at, quien lo hace?
        // el servidor o la base de datos?

        //Si las entradas estan vacias o no tienen un minimo de 2 no tienen sentido procesarlas
        if entries.is_empty() || entries.len() < 2 {
            return Err(TransactionError::InsufficientEntries);// retornar el error
            
        }
        //Validando saldos en 0 o negativos cuando se crea una transaccion 
        //Solo necesito leer por lo que se lo presto
        for entry in &entries  {
            if entry.amount_cents == 0 {
                return Err(TransactionError::ZeroAmountEntry) ;//retornar el error
            }
        }
        let transaction = Transaction { id: id , status: TransactionStatus::Pending, entries:entries, created_at: created_at };

        if !transaction.is_balanced()? {// true ! false false true, por que se puso el !?
            //Se pone el ! porque lo que queremos es arrogar el error si transaction.is_balace da false por lo que invertimos el valor booleano
            return Err(TransactionError::Unbalanced)
            
        }

        Ok(transaction)
        
    }
    
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    InsufficientEntries, // Menos de 2 entradas
    ZeroAmountEntry,     // Entrada con monto 0
    Unbalanced,          // Débitos != Créditos
    ArithmeticOverflow,  // La suma de montos superó el límite de u64
}