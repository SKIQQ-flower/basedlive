import { useStorage } from '@vueuse/core'
import { computed } from 'vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { invoke } from '@tauri-apps/api/core'

interface EmotionFrame {
    frame_type: string;
    path: string;
}

interface Emotion {
    name: string;
    frames: Record<string, EmotionFrame> | null;
}

interface Character {
    name: string;
    emotions: Emotion[];
}

const currentCharacter = useStorage<Character | null>('current-character', null, localStorage, {
    serializer: {
        read: (v: any) => {
            try {
                const parsed = JSON.parse(v)
                if (!parsed) return null
                
                return {
                    ...parsed,
                    emotions: parsed.emotions.map((emotion: Emotion) => ({
                        ...emotion,
                        frames: emotion.frames ? Object.fromEntries(
                            Object.entries(emotion.frames).map(([key, frame]) => [
                                key,
                                {
                                    ...frame,
                                    path: convertFileSrc(frame.path)
                                }
                            ])
                        ) : null
                    }))
                }
            } catch {
                return null
            }
        },
        write: (v: any) => JSON.stringify(v)
    }
})

const recentCharacters = useStorage<string[]>('recent-characters', [], localStorage)

export function useCharacter() {
    const getCharacter = () => currentCharacter.value
    
    const saveCharacter = (character: Character) => {
        console.log('Saving character in state:', character)
        
        currentCharacter.value = {
            ...character,
            emotions: character.emotions.map(emotion => ({
                ...emotion,
                frames: emotion.frames ? Object.fromEntries(
                    Object.entries(emotion.frames).map(([key, frame]) => [
                        key,
                        {
                            ...frame,
                            path: convertFileSrc(frame.path)
                        }
                    ])
                ) : null
            }))
        }
        
        if (character.name && !recentCharacters.value.includes(character.name)) {
            recentCharacters.value = [character.name, ...recentCharacters.value].slice(0, 5)
        }
    }
    
    const loadCharacter = async (path: string) => {
        try {
            console.log('Loading character from path:', path)
            // Fixed: Changed 'path' to 'inputPath' to match the Tauri command's expected parameter
            const character = await invoke('load_character', { inputPath: path }) as Character
            
            currentCharacter.value = {
                ...character,
                emotions: character.emotions.map(emotion => ({
                    ...emotion,
                    frames: emotion.frames ? Object.fromEntries(
                        Object.entries(emotion.frames).map(([key, frame]) => [
                            key,
                            {
                                ...frame,
                                path: convertFileSrc(frame.path)
                            }
                        ])
                    ) : null
                }))
            }
            
            if (character.name && !recentCharacters.value.includes(character.name)) {
                recentCharacters.value = [character.name, ...recentCharacters.value].slice(0, 5)
            }
            
            return currentCharacter.value
        } catch (error) {
            console.error('Error loading character:', error)
            throw error
        }
    }
    
    const clearCharacter = () => {
        currentCharacter.value = null
    }
    
    const hasCharacter = computed(() => currentCharacter.value !== null)
    
    const getRecentCharacters = () => recentCharacters.value
    
    return {
        currentCharacter,
        getCharacter,
        saveCharacter,
        loadCharacter,
        clearCharacter,
        hasCharacter,
        getRecentCharacters
    }
}