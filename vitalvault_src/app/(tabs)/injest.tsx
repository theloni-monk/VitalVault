import { Text, View } from "react-native";
import "../../global.css"

// TODO: upload button {files, camera roll}, camera scan, camera imgage
export default function Injest() {
  return (
    <View
      className="bg-amber-50 flex-1 justify-top"
    >
      <Text className='text-blue-800 text-xl flex-col font-bold self-center w-3/4 pt-10' >upload your documents here</Text>
    </View>
  );
}
