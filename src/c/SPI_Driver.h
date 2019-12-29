/*THE SOFTWARE AND DOCUMENTATION ARE PROVIDED "AS IS" WITHOUT
* WARRANTY OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING WITHOUT
* LIMITATION, ANY WARRANTY OF MERCHANTABILITY, FITNESS FOR A
* PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT SHALL
* IMACS/emBRICK BE LIABLE FOR ANY INCIDENTAL, SPECIAL, INDIRECT OR
* CONSEQUENTIAL DAMAGES, LOST PROFITS OR LOST DATA, COST OF
* PROCUREMENT OF SUBSTITUTE GOODS, TECHNOLOGY OR SERVICES, ANY CLAIMS
* BY THIRD PARTIES (INCLUDING BUT NOT LIMITED TO ANY DEFENSE
* THEREOF), ANY CLAIMS FOR INDEMNITY OR CONTRIBUTION, OR OTHER
* SIMILAR COSTS, WHETHER ASSERTED ON THE BASIS OF CONTRACT, TORT
* (INCLUDING NEGLIGENCE), BREACH OF WARRANTY, OR OTHERWISE. */

/****************************************************************
*   file SPI_Driver.h	                                        *
*                                                               *
* Copyright [C] 2019 by											*
*		fortiss GmbH											*
*       E-Mail: stichling@fortiss.org							*
****************************************************************/
#include "bcm2835.h"
#include <stdio.h>
#include "bBConfigs.h"
#include "bBDefines.h"

#ifndef SPI_DRIVER_H
#define	SPI_DRIVER_H



/****************************************************************
*FUNCTION NAME:		spiInit()									*
*                                                               *
*DESCRIPTION:		The function initializes spi interface		*
*                                                               *
*PARAMETER:			None										*
*                                                               *
*RETURN:			0-initialization is successfully			*
* 					1-Error: initialization is not successfully	*
*                                                               *
****************************************************************/
int spiInit();

/****************************************************************
*FUNCTION NAME:		spi_Close()									*
*                                                               *
*DESCRIPTION:		The function closes spi	interfase			*
*                                                               *
*PARAMETER:			none										*
*                                                               *
*RETURN:			none					                    *
*                                                               *
****************************************************************/
void spiClose(void);

/****************************************************************
*FUNCTION NAME:		bBDoSPI()									*
*                                                               *
*DESCRIPTION:		The function writes and reads 				*
* 					data from the spi interfase					*
*                                                               *
*PARAMETER:			txbuf- Buffer for send data					*
* 					rxbuf- Buffer for receive data				*
*                                                               *
*RETURN:			always 0				                    *
*                                                               *
****************************************************************/
short bBDoSPI(unsigned char * txbuf, unsigned char * rxbuf, short numOfBytes,short postMessageDelay);

#endif	/* SPI_DRIVER_H */
