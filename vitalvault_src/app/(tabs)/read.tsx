import { Text, View } from "react-native";
import "../../global.css"


export default function Read() {
  //WRITEME: on startup launch async workers to verify files
  //WRITEME: on all jobs complete go to About page

  return (
    <View
      className="bg-amber-50 flex-1 justify-top"
    >
      <Text className='text-blue-800 text-xl flex-col font-bold self-center w-3/4 pt-10' >Review your docs here</Text>
    </View>
  );
}
