import { add_amounts_js, greet, total_value_js } from './pkg-node'

const greeting = greet('simon')
console.log('greeting', greeting)

const balances = [
  { denom: 'qwe', amount: '15', price: '0.00123' },
  { denom: 'tyu', amount: '35', price: '1.3339' },
  { denom: 'iop', amount: '12312', price: '0.0000339' },
  { denom: 'asd', amount: '1', price: '12' },
  { denom: 'fgh', amount: '99', price: '99.3339' },
  { denom: 'jkl', amount: '366786785', price: '4' },
  { denom: 'zzx', amount: '345', price: '3.00000003339' },
  { denom: 'cvb', amount: '34234234', price: '1.23000123' },
  { denom: 'fgs', amount: '123111', price: '6.42423' },
  { denom: 'jed', amount: '231231232', price: '0.234234' },
  { denom: '4df', amount: '44698', price: '1.3339' },
  { denom: 'qwe', amount: '15', price: '0.00123' },
  { denom: 'tyu', amount: '35', price: '1.3339' },
  { denom: 'iop', amount: '12312', price: '0.0000339' },
  { denom: 'asd', amount: '1', price: '12' },
  { denom: 'fgh', amount: '99', price: '99.3339' },
  { denom: 'jkl', amount: '366786785', price: '4' },
  { denom: 'zzx', amount: '345', price: '3.00000003339' },
  { denom: 'cvb', amount: '34234234', price: '1.23000123' },
  { denom: 'fgs', amount: '123111', price: '6.42423' },
  { denom: 'jed', amount: '231231232', price: '0.234234' },
  { denom: '4df', amount: '44698', price: '1.3339' },
  { denom: 'qwe', amount: '15', price: '0.00123' },
  { denom: 'tyu', amount: '35', price: '1.3339' },
  { denom: 'iop', amount: '12312', price: '0.0000339' },
  { denom: 'asd', amount: '1', price: '12' },
  { denom: 'fgh', amount: '99', price: '99.3339' },
  { denom: 'jkl', amount: '366786785', price: '4' },
  { denom: 'zzx', amount: '345', price: '3.00000003339' },
  { denom: 'cvb', amount: '34234234', price: '1.23000123' },
  { denom: 'fgs', amount: '123111', price: '6.42423' },
  { denom: 'jed', amount: '231231232', price: '0.234234' },
  { denom: '4df', amount: '44698', price: '1.3339' },
  { denom: 'qwe', amount: '15', price: '0.00123' },
  { denom: 'tyu', amount: '35', price: '1.3339' },
  { denom: 'iop', amount: '12312', price: '0.0000339' },
  { denom: 'asd', amount: '1', price: '12' },
  { denom: 'fgh', amount: '99', price: '99.3339' },
  { denom: 'jkl', amount: '366786785', price: '4' },
  { denom: 'zzx', amount: '345', price: '3.00000003339' },
  { denom: 'cvb', amount: '34234234', price: '1.23000123' },
  { denom: 'fgs', amount: '123111', price: '6.42423' },
  { denom: 'jed', amount: '231231232', price: '0.234234' },
  { denom: '4df', amount: '44698', price: '1.3339' },
]

const totalAmount = add_amounts_js(balances)
console.log(
  'totalAmount js',
  balances.reduce((acc, curr) => acc + parseInt(curr.amount), 0),
)
console.log('totalAmount rust', totalAmount)

const totalValue = total_value_js(balances)
console.log(
  'totalValue js',
  balances.reduce((acc, curr) => acc + parseInt(curr.amount) * parseFloat(curr.price), 0),
)
console.log('totalValue rust', totalValue)
