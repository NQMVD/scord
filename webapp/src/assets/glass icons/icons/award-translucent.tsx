import React, {SVGProps} from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
	secondaryfill?: string;
	strokewidth?: number;
	title?: string;
}

function AwardTranslucent({fill = 'currentColor', secondaryfill, title = 'badge 13', ...props}: IconProps) {
	secondaryfill = secondaryfill || fill;

	return (
		<svg height="24" style={{}} width="24" {...props} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
	<title>{title}</title>
	<g fill="none">
		<path d="M7.00024 16.0002C10.9037 14.6293 13.0983 14.6035 17.0002 16.0002V22.1243C17 22.9242 16.1068 23.4001 15.4426 22.9543L12.0002 20.6418L8.55786 22.9543C7.89363 23.4004 7.0005 22.9243 7.00024 22.1243V16.0002ZM11.1116 3.57544C11.4741 2.80882 12.5254 2.80882 12.8879 3.57544L14.2034 6.35767L17.1516 6.80493C17.9622 6.92786 18.2867 7.96309 17.7014 8.55884L15.5657 10.7307L16.0696 13.7952C16.2078 14.6361 15.3589 15.2768 14.6331 14.8792L12.0002 13.4348L9.36743 14.8792C8.64156 15.2771 7.79165 14.6362 7.92993 13.7952L8.43384 10.7307L6.2981 8.55884C5.71307 7.96307 6.03748 6.92789 6.8479 6.80493L9.79614 6.35767L11.1116 3.57544Z" fill="url(#ms5yps25911752500502766-7825576_award_existing_0_1d5vz9kue)" mask="url(#ms5yps25911752500502766-7825576_award_mask_jj2jdngeg)" data-glass="origin"/>
		<path clipPath="url(#ms5yps25911752500502766-7825576_award_clipPath_wup8q1dbv)" d="M7.00024 16.0002C10.9037 14.6293 13.0983 14.6035 17.0002 16.0002V22.1243C17 22.9242 16.1068 23.4001 15.4426 22.9543L12.0002 20.6418L8.55786 22.9543C7.89363 23.4004 7.0005 22.9243 7.00024 22.1243V16.0002ZM11.1116 3.57544C11.4741 2.80882 12.5254 2.80882 12.8879 3.57544L14.2034 6.35767L17.1516 6.80493C17.9622 6.92786 18.2867 7.96309 17.7014 8.55884L15.5657 10.7307L16.0696 13.7952C16.2078 14.6361 15.3589 15.2768 14.6331 14.8792L12.0002 13.4348L9.36743 14.8792C8.64156 15.2771 7.79165 14.6362 7.92993 13.7952L8.43384 10.7307L6.2981 8.55884C5.71307 7.96307 6.03748 6.92789 6.8479 6.80493L9.79614 6.35767L11.1116 3.57544Z" fill="url(#ms5yps25911752500502766-7825576_award_existing_0_1d5vz9kue)" data-glass="clone"/>
		<path d="M12 1C16.6944 1 20.5 4.80558 20.5 9.5C20.5 14.1944 16.6944 18 12 18C7.30558 18 3.5 14.1944 3.5 9.5C3.5 4.80558 7.30558 1 12 1Z" fill="url(#ms5yps25911752500502766-7825576_award_existing_1_zhbnze0qb)" data-glass="blur"/>
		<path d="M12 1C16.6944 1 20.5 4.80558 20.5 9.5C20.5 14.1944 16.6944 18 12 18C7.30558 18 3.5 14.1944 3.5 9.5C3.5 4.80558 7.30558 1 12 1ZM12 1.75C7.71979 1.75 4.25 5.21979 4.25 9.5C4.25 13.7802 7.71979 17.25 12 17.25C16.2802 17.25 19.75 13.7802 19.75 9.5C19.75 5.21979 16.2802 1.75 12 1.75Z" fill="url(#ms5yps25911752500502766-7825576_award_existing_2_920iaalxf)"/>
		<defs>
			<linearGradient id="ms5yps25911752500502766-7825576_award_existing_0_1d5vz9kue" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="-.5" y2="23.126">
				<stop stopColor="#575757"/>
				<stop offset="1" stopColor="#1f1e1e"/>
			</linearGradient>
			<linearGradient id="ms5yps25911752500502766-7825576_award_existing_1_zhbnze0qb" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="18">
				<stop stopColor="#e3e3e559" stopOpacity=".6"/>
				<stop offset="1" stopColor="#bbbbc0af" stopOpacity=".6"/>
			</linearGradient>
			<linearGradient id="ms5yps25911752500502766-7825576_award_existing_2_920iaalxf" gradientUnits="userSpaceOnUse" x1="12" x2="12" y1="1" y2="10.845">
				<stop stopColor="#ffffff99"/>
				<stop offset="1" stopColor="#ffffff99" stopOpacity="0"/>
			</linearGradient>
			<clipPath id="ms5yps25911752500502766-7825576_award_clipPath_wup8q1dbv">
				<path d="M12 1C16.6944 1 20.5 4.80558 20.5 9.5C20.5 14.1944 16.6944 18 12 18C7.30558 18 3.5 14.1944 3.5 9.5C3.5 4.80558 7.30558 1 12 1Z" fill="url(#ms5yps25911752500502766-7825576_award_existing_1_zhbnze0qb)"/>
			</clipPath>
			<mask id="ms5yps25911752500502766-7825576_award_mask_jj2jdngeg">
				<rect height="100%" width="100%" fill="#FFF"/>
				<path d="M12 1C16.6944 1 20.5 4.80558 20.5 9.5C20.5 14.1944 16.6944 18 12 18C7.30558 18 3.5 14.1944 3.5 9.5C3.5 4.80558 7.30558 1 12 1Z" fill="#000"/>
			</mask>
		</defs>
	</g>
</svg>
	);
};

export default AwardTranslucent;