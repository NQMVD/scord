import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function Video({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M21.8291 7.08496C22.8263 6.58642 23.9998 7.31188 24 8.42676V15.5732C23.9998 16.6881 22.8263 17.4136 21.8291 16.915L15.5781 13.7891C14.104 13.052 14.104 10.948 15.5781 10.2109L21.8291 7.08496ZM5.5 6C6.8807 6.00002 8 7.1193 8 8.5C7.99998 9.88068 6.88069 11 5.5 11C4.1193 11 3.00002 9.88069 3 8.5C3 7.11929 4.11929 6 5.5 6Z" fill="url(#1752500502812-5740093_video_existing_0_dbpfea8jj)" mask="url(#1752500502812-5740093_video_mask_a931ih6v4)" data-glass="origin"/>
		<path clipPath="url(#1752500502812-5740093_video_clipPath_vtj9f1llm)" d="M21.8291 7.08496C22.8263 6.58642 23.9998 7.31188 24 8.42676V15.5732C23.9998 16.6881 22.8263 17.4136 21.8291 16.915L15.5781 13.7891C14.104 13.052 14.104 10.948 15.5781 10.2109L21.8291 7.08496ZM5.5 6C6.8807 6.00002 8 7.1193 8 8.5C7.99998 9.88068 6.88069 11 5.5 11C4.1193 11 3.00002 9.88069 3 8.5C3 7.11929 4.11929 6 5.5 6Z" fill="url(#1752500502812-5740093_video_existing_0_dbpfea8jj)" data-glass="clone"/>
		<path d="M14 4C16.2091 4 18 5.79086 18 8V16C18 18.2091 16.2091 20 14 20H5C2.79086 20 1 18.2091 1 16V8C1 5.79086 2.79086 4 5 4H14ZM5.5 7C4.67157 7 4 7.67157 4 8.5C4 9.32843 4.67157 10 5.5 10C6.32843 10 7 9.32843 7 8.5C7 7.67157 6.32843 7 5.5 7Z" fill="url(#1752500502812-5740093_video_existing_1_w4sfz2p1h)" data-glass="blur"/>
		<path d="M14 4C16.2091 4 18 5.79086 18 8V16C18 18.2091 16.2091 20 14 20H5C2.79086 20 1 18.2091 1 16V8C1 5.79086 2.79086 4 5 4H14ZM5 4.75C3.20508 4.75 1.75 6.20508 1.75 8V16L1.75391 16.167C1.84082 17.8843 3.26108 19.25 5 19.25H14L14.167 19.2461C15.829 19.162 17.162 17.829 17.2461 16.167L17.25 16V8L17.2461 7.83301C17.162 6.17099 15.829 4.83802 14.167 4.75391L14 4.75H5Z" fill="url(#1752500502812-5740093_video_existing_2_k425tjp5d)"/>
		<defs>
			<linearGradient id="1752500502812-5740093_video_existing_0_dbpfea8jj" gradientUnits="userSpaceOnUse" x1="13.5" x2="13.5" y1="6" y2="17.075">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#151515"/>
			</linearGradient>
			<linearGradient id="1752500502812-5740093_video_existing_1_w4sfz2p1h" gradientUnits="userSpaceOnUse" x1="9.5" x2="9.5" y1="4" y2="20">
				<stop stopColor="#E3E3E5" stopOpacity=".6"/>
				<stop offset="1" stopColor="#BBBBC0" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="1752500502812-5740093_video_existing_2_k425tjp5d" gradientUnits="userSpaceOnUse" x1="9.5" x2="9.5" y1="4" y2="13.266">
				<stop stopColor="#fff"/>
				<stop offset="1" stopColor="#fff" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="1752500502812-5740093_video_clipPath_vtj9f1llm">
				<path d="M14 4C16.2091 4 18 5.79086 18 8V16C18 18.2091 16.2091 20 14 20H5C2.79086 20 1 18.2091 1 16V8C1 5.79086 2.79086 4 5 4H14ZM5.5 7C4.67157 7 4 7.67157 4 8.5C4 9.32843 4.67157 10 5.5 10C6.32843 10 7 9.32843 7 8.5C7 7.67157 6.32843 7 5.5 7Z" fill="url(#1752500502812-5740093_video_existing_1_w4sfz2p1h)"/>
			</clipPath>
			<mask id="1752500502812-5740093_video_mask_a931ih6v4">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M14 4C16.2091 4 18 5.79086 18 8V16C18 18.2091 16.2091 20 14 20H5C2.79086 20 1 18.2091 1 16V8C1 5.79086 2.79086 4 5 4H14ZM5.5 7C4.67157 7 4 7.67157 4 8.5C4 9.32843 4.67157 10 5.5 10C6.32843 10 7 9.32843 7 8.5C7 7.67157 6.32843 7 5.5 7Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default Video;