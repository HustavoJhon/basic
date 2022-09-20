DECLARE @importe DECIMAL(18,2),
	@CuentaOrigen VARCHAR(12),
    @CuentaDestino VARCHAR(12)
    
/* Asignamos el importe de la transferencia* y las cuentas de origen y destino*/
SET @importe = 50
SET @CuentaOrigen = '200700000001'
SET @CuentaDestino = '200700000002'

/* Descontamos el importe de la cuenta origen */
UPDATE CUENTAS
SET SALDO = SALDO - @importe
WHERE NUMCUENTA = @CuentaOrigen

/* Registramos el movimiento */
INSERT INTO MOVIMIENTOS
(IDCUENTA, SALDO_ANTERIOR, SALDO_POSTERIOR, IMPORTE,FXMOVIMIENTO)
SELECT 
IDCUENTA, SALDO + @importe, SALDO, @importe, getdate()
FROM CUENTAS
WHERE NUMCUENTA = @CuentaOrigen

/* Incrementamos el importe de la cuenta destino */
UPDATE CUENTAS
SET SALDO = SALDO + @importe
WHERE NUMCUENTA = @CuentaDestino

/* Registramos el movimiento */
INSERT INTO MOVIMIENTOS
(IDCUENTA, SALDO_ANTERIOR, SALDO_POSTERIOR, IMPORTE,FXMOVIMIENTO)
SELECT
IDCUENTA, SALDO - @importe, SALDO, @importe, getdate()
FROM CUENTAS
WHERE NUMCUENTA = @CuentaDestino
