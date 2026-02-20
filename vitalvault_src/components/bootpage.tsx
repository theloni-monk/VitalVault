import { View, Image, Text, TouchableNativeFeedback } from 'react-native';
import { setData, getData, hasData, removeData } from '@choochmeque/tauri-plugin-biometry-api';


async function tryauth(unlockhook: () => void) {


  // Check if data exists
  const exists = await hasData({
    domain: 'com.vitalvault',
    name: 'vault_key'
  });

  // Retrieve data (will prompt for biometric authentication)
  if (!exists) {
    // Store data with biometric protection
    let key = new Uint8Array(67); 
    crypto.getRandomValues(key)
    await setData({
    domain: 'com.vitalvault',
    name: 'vault_key',
    data: Buffer.from(key).toString('base64')
  });
  }
  try{
    const response = await getData({
      domain: 'com.vitalvault',
      name: 'vault_key',
      reason: 'Access your vault of vital info'
    });
    //WRITEME: call tauri backend connection to duckdb here with response.data

    unlockhook()
  } catch (e) {
    console.log("Auth failed", e);
    return;
  }
}


export default function Bootpage({ unlockhook }: { unlockhook: () => void }) {
  return (
    <View className='flex-1 flex-col bg-amber-900'>
        <Image className='align-middle self-center mt-60'
          source={require("../assets/landing_logo.svg")}/>
          <TouchableNativeFeedback onPress={async () => await tryauth(unlockhook)}>
            <View className='align-middle self-center mt-40 bg-blue-500 p-4 rounded-lg flex-row'>
              <Image source={require("../assets/unlock.svg")}/> 
              <Text>Unlock</Text>
            </View>
          </TouchableNativeFeedback>
    </View>
    )
}