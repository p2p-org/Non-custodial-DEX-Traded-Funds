export const sleep: (ms: number) => Promise<void> = (ms: number) =>
  new Promise((resolve) => setTimeout(resolve, ms));

export const shortAddress = (address: string) => {
  return `${address.slice(0, 4)}…${address.slice(-4)}`;
};

export const moneyFormat = (amount: number) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
  }).format(amount);
};
