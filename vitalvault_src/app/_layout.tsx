
import "../global.css";

import { useFonts } from 'expo-font';
import { Stack } from 'expo-router';
import { Text, View } from "react-native";
import { useEffect, useState} from 'react';
import FontAwesome5 from '@expo/vector-icons/FontAwesome5'


export default function RootLayout() {

  const [allLoaded, setAllLoaded] = useState(false);
  const [mockLoad, setMockLoaded] = useState(false);
  const [fontLoaded] = useFonts({
    SpaceMono: require('../assets/fonts/SpaceMono-Regular.ttf'),
  });
  
  //on mount
  useEffect(() => {
    // Fake loading, eventually replaced with database access and model weight access
    const timeoutId = setTimeout(() => {
      setMockLoaded(true);
    }, 5000);

    // Cleanup function to clear the timeout if the component unmounts
    return () => clearTimeout(timeoutId);
  }, []); // Empty dependency array ensures the effect runs only once

  
  // on change
  useEffect(() => {
    setAllLoaded(mockLoad && fontLoaded)
  }, [mockLoad, fontLoaded]);

  if (!allLoaded)
    return (
      <View className="flex-1 flex-col bg-amber-50">
        <View className="justify-center align-top h-full">
          <Text className="text-blue-800 font-bold text-2xl self-center">Loading stuff...{"\n"}</Text>
          FIXME: no whatever
          <FontAwesome5 className = "color-pink-200 animate-pulse self-center" size="1000" name="heartbeat"/>
        </View>
      </View>
    );
  
  return (
  <Stack>
    <Stack.Screen name="(tabs)" options={{ headerShown: false }} />
  </Stack>
  );
}
