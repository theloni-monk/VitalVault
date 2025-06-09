
import "../global.css"
import { View } from "react-native";
export default function LoadingAnim() {
    return (
        <View className="relative flex size-10 items-center justify-center">
            <View className="absolute size-10 animate-ping rounded-full border-4 border-current text-cyan-600 dark:text-cyan-400"></View>
            <View className="absolute h-6 w-1 animate-[spin_1.5s_linear_infinite] bg-teal-600 dark:bg-teal-400"></View>
            <View className="absolute h-6 w-1 rotate-45 animate-[spin_1.5s_linear_infinite] bg-teal-600 [animation-delay:-0.2s] dark:bg-teal-400"></View>
            <View className="absolute h-6 w-1 rotate-90 animate-[spin_1.5s_linear_infinite] bg-teal-600 [animation-delay:-0.4s] dark:bg-teal-400"></View>
        </View>);

}