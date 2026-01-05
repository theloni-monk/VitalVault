import { Text, View } from "react-native";
import "../../global.css"

//TODO: search bar, past 3 searches on hover, recent files, pinned files
export default function Find() {
  return (
    <View
      className="bg-amber-50 flex-1 justify-top"
    >
      <Text className='text-blue-800 text-xl flex-col font-bold self-center w-3/4 pt-10' >Here you search for stuff</Text>
    </View>
  );
}
