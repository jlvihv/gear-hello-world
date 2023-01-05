// let nodeAddress = process.env.REACT_APP_NODE_ADDRESS;
let nodeAddress = process.env.PROVIDER_ADDRESS;
if (!nodeAddress || nodeAddress === "") {
  nodeAddress = "wss://rpc-node.gear-tech.io";
}
const ADDRESS = {
  NODE: nodeAddress,
  DAPPS_API: process.env.REACT_APP_DAPPS_API_ADDRESS as string,
};

const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
};

const SUBHEADING = {
  LOGIN: 'In order to start, please login.',
};

export { ADDRESS, LOCAL_STORAGE, SUBHEADING };
