
import "../global.css";

import { useFonts } from 'expo-font';
import { Stack } from 'expo-router';
import { Text, View } from "react-native";
import LoadingAnim from "../components/loadingspin";
import { verifyInstallation } from 'nativewind';
import { useEffect, useState} from 'react';

export default function RootLayout() {
  verifyInstallation()

  const [allLoaded, setAllLoaded] = useState(false);
  const [toutloaded, setTOutLoaded] = useState(false);
  const [fontLoaded] = useFonts({
    SpaceMono: require('../assets/fonts/SpaceMono-Regular.ttf'),
  });
  
  //on mount
  useEffect(() => {
    // Use setTimeout to update the message after 2000 milliseconds (2 seconds)
    const timeoutId = setTimeout(() => {
      setTOutLoaded(true);
    }, 5000);

    // Cleanup function to clear the timeout if the component unmounts
    return () => clearTimeout(timeoutId);
  }, []); // Empty dependency array ensures the effect runs only once

  
  // on change
  useEffect(() => {
    setAllLoaded(toutloaded && fontLoaded)
  }, [toutloaded, fontLoaded]);

  if (!allLoaded)
    return (
      <View className="align-middle justify-center bg-amber-100">
        <Text className="text-red-500 size-10">Loading stuff...</Text>
        <LoadingAnim />
      </View>
    );
  
  return (
  <Stack>
    <Stack.Screen name="(tabs)" options={{ headerShown: false }} />
  </Stack>
  );
}
